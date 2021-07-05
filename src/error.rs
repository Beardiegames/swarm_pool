use std::fmt;


pub const MAX_SPAWNS_REACHED: SwarmError = SwarmError (
    "MAX_SPAWNS_REACHED: Unable to spawn, all availeble spawns are in use!"
);
pub const KILL_DURING_UPDATE: SwarmError = SwarmError (
    "KILL_DURING_UPDATE: Cannot kill a spawn inside an update loop, use delayed_kill() instead!"
);
pub const NO_SPAWN_TO_KILL: SwarmError = SwarmError (
    "NO_SPAWN_TO_KILL: There are no active spawn to kill"
);
pub const UNKNOWN: SwarmError = SwarmError (
    "UNKNOWN: An error occured where no error was expected!"
);


pub struct SwarmError (&'static str);

impl fmt::Debug for SwarmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("SwarmError::{}", self.0))
    }
}


// pub enum SwarmError {
//     MaxSpawnsReached,
//     KillDuringUpdateLoop,
//     NoSpawnsToKill,
//     UnknownError,
// }

// impl fmt::Debug for SwarmError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let (arg_a, arg_b) = match self {
//             MaxSpawnsReached => (
//                 "MaxSpawnsReached", 
                
//             ),
//             KillDuringUpdateLoop => (
//                 "KillDuringUpdateLoop",
//                 "Cannot kill a spawn inside the update loop, use delayed_kill instead!"
//             ),
//             NoSpawnsToKill => (
//                 "NoSpawnsToKill",
                
//             ),
//             UnknownError,
//         }
//         f.write_fmt(format_args!("SwarmError::{} -> {}", arg_a, arg_b))
//     }
// }