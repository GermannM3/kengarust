use libp2p::{Swarm, Multiaddr, identity};

pub struct ModelNetwork {
    swarm: Swarm<NetworkBehaviour>,
}

impl ModelNetwork {
    pub async fn broadcast(&mut self, message: ModelMessage) {
        let topic = String::from("neuro-exchange");
        self.swarm.publish(&topic, message.encode()).unwrap();
    }

    pub async fn listen(&mut self) -> impl Stream<Item = ModelMessage> {
        self.swarm.subscribe("neuro-exchange").await
    }
}
