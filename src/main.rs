use futures::StreamExt;
use libp2p::{tcp::TcpConfig, PeerId, identity::{self}, floodsub::{Topic, Floodsub, FloodsubEvent}, development_transport, mdns::{Mdns, MdnsEvent, MdnsConfig}, NetworkBehaviour, swarm::{NetworkBehaviourEventProcess, SwarmBuilder}, core::upgrade, Transport, noise::{NoiseConfig, X25519Spec, Keypair}, mplex, Swarm, rendezvous::client::Behaviour,};
use log::{info, warn, error};
use tokio::io::AsyncBufReadExt;


#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct Chat {
    dns: Mdns,
    messager: Floodsub,
    // history: ,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Chat {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(nodes) => {
                println!("New node found!");
                //New node found!
                for (peer, _) in nodes {
                    self.messager.add_node_to_partial_view(peer);
                }
            },
            MdnsEvent::Expired(nodes) => {
                //Do we *need* to handle this? What's the risk?
                //They'll just build up right...
                println!("Node disconnected!");
                for (peer, _) in nodes {
                    if !self.dns.has_node(&peer) { //Why this check?
                        self.messager.remove_node_from_partial_view(&peer);
                    }
                }
            },
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for Chat {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                println!("{} said: {}", msg.source, String::from_utf8_lossy(&msg.data));
            },
            e => {
                //TODO, learn the specifics about subbing
                println!("Recieved unsupported event type. {:?}", e);
            } ,
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


    let mut behaviour = Chat {
        dns: Mdns::new(MdnsConfig::default())
            .await
            .expect("unable to create mdns"),
        messager: Floodsub::new(peer_id),
    };

    let topic = Topic::new("sylo");
    behaviour.messager.subscribe(topic.clone());

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    loop {

        tokio::select! {
            line = stdin.next_line() => {
                //Oh hey, our user typed a message!
                //Lets broadcast it to the others
                if let Some(input_line) = line.expect("a valid line") {
                    swarm
                        .behaviour_mut()
                        .messager
                        .publish(topic.clone(), input_line.as_bytes());
                }
            },
            event = swarm.select_next_some() => {
                info!("Swarm event: {:?}", event);
            },
        }


    }
}