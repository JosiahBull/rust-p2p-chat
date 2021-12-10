
mod history;

use std::collections::HashMap;
use futures::StreamExt;
use libp2p::{tcp::TcpConfig, PeerId, identity::{self}, floodsub::{Topic, Floodsub, FloodsubEvent}, mdns::{Mdns, MdnsEvent, MdnsConfig}, NetworkBehaviour, swarm::NetworkBehaviourEventProcess, core::upgrade, Transport, noise::{NoiseConfig, X25519Spec, Keypair}, mplex, Swarm};
use log::{info, error};
use serde::{Serialize, Deserialize};
use tokio::{io::AsyncBufReadExt, sync::mpsc};
use history::History;

//TODOS
//- Bootstrap nodes, rather than mDNS
//- RequestReply to send Username/History to interested nodes rather than crapping over the entire network
//- Gossipsub vs Floodsub? Apparently Gossipsub is the more efficent of the two.

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct Chat {
    dns: Mdns,
    messager: Floodsub,
    #[behaviour(ignore)]
    usernames: HashMap<PeerId, String>,
    #[behaviour(ignore)]
    history: History<Message>,
    #[behaviour(ignore)]
    username: String,
    #[behaviour(ignore)]
    peer_id: String,
    #[behaviour(ignore)]
    responder: mpsc::UnboundedSender<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Message,
    HistoryRequest,
    HistoryResponse,
    UsernameRequest,
    UsernameResponse,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    /// The type of message this is.
    message_type: MessageType,
    /// The data contained within the message, represented as a vector of bytes.
    data: Vec<u8>, //It would be better to use a borrowed value here, as vecs heap allocate
    /// The intended recipient of the message, a PeerId encoded as a string.
    addressee: Option<String>,
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
            },
            MdnsEvent::Expired(nodes) => {
                //Do we *need* to handle this? What's the risk?
                //They'll just build up right...
                for (peer, addr) in nodes {
                    if !self.dns.has_node(&peer) { //Why this check?
                        info!("Peer {} disconnected at {}", peer, addr);
                        self.messager.remove_node_from_partial_view(&peer);
                    }
                }
            },
        }
    }
}

fn send_response(message: Message, sender: mpsc::UnboundedSender<Message>) {
    tokio::spawn(async move {
        if let Err(e) = sender.send(message) {
            error!("error sending response via channel {}", e);
        }
    });
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

                    let username: String = self.usernames
                        .get(&raw_data.source)
                        .unwrap_or(&String::from("Anon"))
                        .to_owned();

                    match message.message_type {
                        MessageType::Message => {
                            println!("{}: {}", username, String::from_utf8_lossy(&message.data));

                            //Store message in history
                            self.history.insert(message);
                        },
                        MessageType::UsernameRequest => {
                            info!("Username request received");
                            let message: Message = Message {
                                message_type: MessageType::UsernameResponse,
                                data: self.username.as_bytes().to_vec(),
                                addressee: Some(raw_data.source.to_string()),
                            };
                            send_response(message, self.responder.clone());
                        },
                        MessageType::UsernameResponse => {
                            info!("Username recieved");
                            if !self.usernames.contains_key(&raw_data.source) {
                                let username = String::from_utf8_lossy(&message.data);
                                self.usernames.insert(raw_data.source, username.to_string());
                                println!("{} joined the chat!", username);
                            }
                        },
                        MessageType::HistoryRequest => {
                            error!("User attempted to collect the history of this chat, this is not yet supported.");
                        },
                        MessageType::HistoryResponse => {
                            let data: Vec<Message> = bincode::deserialize(&message.data).unwrap(); //TODO
                            for message in data {
                                self.history.insert(message);
                            }
                        }
                    }
                } else {
                    error!("Unable to decode message! Due to {:?}", deser.unwrap_err());
                }
            },
            FloodsubEvent::Subscribed { peer_id, topic: _ } => {
                let message: Message = Message {
                    message_type: MessageType::UsernameRequest,
                    data: vec![],
                    addressee: Some(peer_id.to_string()),
                };
                send_response(message, self.responder.clone());
            },
            FloodsubEvent::Unsubscribed { peer_id, topic: _ } => {
                let name = self.usernames.remove(&peer_id).unwrap_or(String::from("Anon"));
                println!("{} has left the chat.", name);
            },
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
    let auth_keys = Keypair::<X25519Spec>::new().into_authentic(&id_keys).expect("unable to create authenticated keys");
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

    let username = stdin.next_line().await.expect("a valid username").unwrap_or(String::from("anon")).trim().to_owned();

    let mut behaviour = Chat {
        dns: Mdns::new(MdnsConfig::default())
            .await
            .expect("unable to create mdns"),
        messager: Floodsub::new(peer_id),
        history: History::new(),
        usernames: HashMap::default(),
        username,
        peer_id: peer_id.to_string(),
        responder: response_sender,
    };

    let topic = Topic::new("sylo");
    behaviour.messager.subscribe(topic.clone());

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    //Request history
    let history_request = Message {
        message_type: MessageType::HistoryRequest,
        data: vec![],
        addressee: None,
    };

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
                    };
                    let bytes = bincode::serialize(&message).unwrap();
                    swarm
                        .behaviour_mut()
                        .messager
                        .publish(topic.clone(), bytes);
                }
            },
            event = swarm.select_next_some() => {
                info!("Swarm event: {:?}", event);
            },
            response = response_rcv.recv() => {
                if let Some(message) = response {
                    let bytes = bincode::serialize(&message).unwrap();
                    swarm
                        .behaviour_mut()
                        .messager
                        .publish(topic.clone(), bytes);
                }
            }
        }
    }
}