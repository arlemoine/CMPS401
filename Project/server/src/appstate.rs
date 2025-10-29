    use std::{
        sync::{Arc, RwLock},
        collections::HashMap,
    };

    use crate::gameroom::GameRoom;
    
    // Holds state of the application backend
    #[derive(Clone, Default)]
    pub struct AppState {
        pub rooms: Arc<RwLock<HashMap<String, GameRoom>>>, // key, value pair (room_id: String, room_object: GameRoom)
    }