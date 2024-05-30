use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

// To communicate between the main world and the render world we need a channel.
// Since the main world and render world run in parallel, there will always be a frame of latency
// between the data sent from the render world and the data received in the main world
//
// frame n => render world sends data through the channel at the end of the frame
// frame n + 1 => main world receives the data
//
// Receiver and Sender are kept in resources because there is single camera and single target
// That's why there is single images role, if you want to differentiate images
// from different cameras, you should keep Receiver in ImageCopier and Sender in ImageToSave
// or send some id with data

/// This will receive asynchronously any data sent from the render world
#[derive(Resource, Deref)]
pub struct MainWorldReceiver(pub Receiver<Vec<u8>>);

/// This will send asynchronously any data to the main world
#[derive(Resource, Deref)]
pub struct RenderWorldSender(pub Sender<Vec<u8>>);
