mod history;
mod state;

use futures::StreamExt;
use history::History;
use libp2p::{
    core::upgrade,
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity::{self},
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::NetworkBehaviourEventProcess,
    tcp::TcpConfig,
    NetworkBehaviour, PeerId, Swarm, Transport,
};
use log::{error, info};
use state::{Message, MessageType, State};
use std::{collections::HashMap, process};
use tokio::{io::AsyncBufReadExt, sync::mpsc, signal::ctrl_c};

//TODOS
//- Bootstrap nodes, rather than mDNS
//- RequestReply to send Username/History to interested nodes rather than crapping over the entire network
//- Gossipsub vs Floodsub? Apparently Gossipsub is the more efficent of the two.

/// Transmit a response to other peers utilising the channels
fn send_response(message: Message, sender: mpsc::UnboundedSender<Message>) {
    tokio::spawn(async move {
        if let Err(e) = sender.send(message) {
            error!("error sending response via channel {}", e);
        }
    });
}

/// Send a message using the swarm
fn send_message(message: &Message, swarm: &mut Swarm<Chat>, topic: &Topic) {
    let bytes = bincode::serialize(message).unwrap();
    swarm
        .behaviour_mut()
        .messager
        .publish(topic.clone(), bytes);
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct Chat {
    dns: Mdns,
    messager: Floodsub,
    #[behaviour(ignore)]
    state: State,
    #[behaviour(ignore)]
    peer_id: String,
    #[behaviour(ignore)]
    responder: mpsc::UnboundedSender<Message>,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Chat {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(nodes) => {
                //New node found!
                for (peer, addr) in nodes {
                    //XXX Do we need a condition check to see if this node already exists?
                    info!("Peer {} found at {}", peer, addr);
                    self.messager.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(nodes) => {
                //Do we *need* to handle this? What's the risk?
                //They'll just build up right...
                for (peer, addr) in nodes {
                    if !self.dns.has_node(&peer) {
                        //Why this check?
                        info!("Peer {} disconnected at {}", peer, addr);
                        self.messager.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for Chat {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(raw_data) => {
                //Parse the message as bytes
                let deser = bincode::deserialize::<Message>(&raw_data.data);
                if let Ok(message) = deser {
                    if let Some(user) = &message.addressee {
                        if *user != self.peer_id.to_string() {
                            return; //Don't process messages not intended for us.
                        }
                    }

                    match message.message_type {
                        MessageType::Message => {
                            let username: String =
                                self.state.get_username(&raw_data.source.to_string());
                            println!("{}: {}", username, String::from_utf8_lossy(&message.data));

                            //Store message in history
                            self.state.history.insert(message);
                        }
                        MessageType::State => {
                            info!("History recieved!");
                            let data: State = bincode::deserialize(&message.data).unwrap();
                            self.state.merge(data);
                        }
                    }
                } else {
                    error!("Unable to decode message! Due to {:?}", deser.unwrap_err());
                }
            }
            FloodsubEvent::Subscribed { peer_id, topic: _ } => {
                //Send our state to new user
                info!("Sending stage to {}", peer_id);
                let message: Message = Message {
                    message_type: MessageType::State,
                    data: bincode::serialize(&self.state).unwrap(),
                    addressee: Some(peer_id.to_string()),
                    source: self.peer_id.to_string(),
                };
                send_response(message, self.responder.clone());
            }
            FloodsubEvent::Unsubscribed { peer_id, topic: _ } => {
                let name = self
                    .state
                    .usernames
                    .remove(&peer_id.to_string())
                    .unwrap_or(String::from("Anon"));
                println!("{} has left the chat.", name);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer ID: {}", peer_id);

    //TODO, note I have no idea what this does.
    // More reading is required!
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&id_keys)
        .expect("unable to create authenticated keys");

    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    //Generate channel
    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    //Ask the user to generate a username for themselves
    print!("Please enter a username: \n");
    let username = stdin
        .next_line()
        .await
        .expect("a valid username")
        .unwrap_or(String::from("anon"))
        .trim()
        .to_owned();

    let mut behaviour = Chat {
        dns: Mdns::new(MdnsConfig::default())
            .await
            .expect("unable to create mdns"),
        messager: Floodsub::new(peer_id),
        state: State {
            history: History::new(),
            usernames: HashMap::from([(peer_id.to_string(), username)]),
        },
        peer_id: peer_id.to_string(),
        responder: response_sender,
    };

    let topic = Topic::new("sylo");
    behaviour.messager.subscribe(topic.clone());

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                //Oh hey, our user typed a message!
                //Lets broadcast it to the others
                if let Some(input_line) = line.expect("a valid line") {
                    let message: Message = Message {
                        message_type: MessageType::Message,
                        data: input_line.as_bytes().to_vec(),
                        addressee: None,
                        source: peer_id.to_string(),
                    };

                    send_message(&message, &mut swarm, &topic);

                    //Log Message
                    swarm
                        .behaviour_mut()
                        .state
                        .history
                        .insert(message);
                }
            },
            event = swarm.select_next_some() => {
                info!("Swarm event: {:?}", event);
            },
            response = response_rcv.recv() => {
                if let Some(message) = response {
                    send_message(&message, &mut swarm, &topic);
                }
            },
            event = ctrl_c() => {
                if let Err(e) = event {
                    println!("Failed to register interrupt handler {}", e);
                }
                break;
            }
        }
    }

    swarm.behaviour_mut().messager.unsubscribe(topic);

    //HACK workaround to force the unsubscribe to actually send.
    //Annoying as it causes a delay when closing the application.
    swarm.select_next_some().await;

    process::exit(0);
}