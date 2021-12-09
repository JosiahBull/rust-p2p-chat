use libp2p::{tcp::TcpConfig, PeerId, identity::{self}, floodsub::{Topic, Floodsub, FloodsubEvent}, development_transport, mdns::{Mdns, MdnsEvent}, NetworkBehaviour, swarm::{NetworkBehaviourEventProcess, SwarmBuilder}, core::upgrade, Transport, noise::{NoiseConfig, X25519Spec, Keypair}, mplex, Swarm, rendezvous::client::Behaviour,};
use log::{info, warn, error};



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
                //New node found!
                for (peer, _) in nodes {
                    self.messager.add_node_to_partial_view(peer);
                }
            },
            MdnsEvent::Expired(nodes) => {
                //Do we *need* to handle this? What's the risk.
                //They'll just build up right...
                //To be discussed w/ paul :p
                for (peer, _) in nodes {
                    if !self.dns.has_node(&peer) {
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
            e => warn!("Recieved unsupported event type. {:?}", e), //TODO, learn the specifics about subbing
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    info!("Local peer ID: {}", peer_id);

    //TODO, note I have no idea what this does.
    // More reading is required!
    let auth_keys = Keypair::<X25519Spec>::new().into_authentic(&id_keys).expect("unable to create authenticated keys");
    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();


    let mut behaviour = Chat {
        dns: Mdns::new(Default::default()) //wtf is this syntax?!
            .await
            .expect("unable to create mdns"),
        messager: Floodsub::new(peer_id),
    };

    let topic = Topic::new("sylo");
    behaviour.messager.subscribe(topic);

    let mut swarm = SwarmBuilder::new(transport, behaviour, peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    //What alternative swarms are there?
    //TODO reading
    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can't get a local socket"),
    )
    .expect("swarm can be started");




    Ok(())
}