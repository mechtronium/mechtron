#![allow(warnings)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;


use serde::{Deserialize, Serialize};



pub mod inlet{
    use mesh_portal::error::MsgErr;
    use mesh_portal::version::latest::artifact::{ArtifactRequest, ArtifactResponse};
    use mesh_portal::version::latest::messaging::{Request, Response};
    use mesh_portal::version::latest::portal;
    use mesh_portal::version::latest::portal::Exchanger;
    use mesh_portal::version::latest::log;
    use mesh_portal::version::latest::log::Log;
    use serde::{Serialize,Deserialize};



    #[derive(Debug,Clone,Serialize,Deserialize)]
    pub enum Frame {
        Log(Log),
        ArtifactRequest(Exchanger<ArtifactRequest>),
    }

    impl Into<portal::inlet::Frame> for Frame {
        fn into(self) -> portal::inlet::Frame {
            match self {
                Frame::Log(log) => {
                    portal::inlet::Frame::Log(log.into())
                }
                Frame::ArtifactRequest(request) => {
                    portal::inlet::Frame::Artifact(request)
                }
            }
        }
    }
}


pub mod outlet {
    use mesh_portal::error::MsgErr;
    use mesh_portal::version::latest::artifact::ArtifactResponse;
    use mesh_portal::version::latest::config::Assign;
    use mesh_portal::version::latest::id::Version;
    use mesh_portal::version::latest::messaging::{Request, Response};
    use mesh_portal::version::latest::portal;
    use mesh_portal::version::latest::portal::Exchanger;
    use serde::{Serialize,Deserialize};
    use core::convert::TryFrom;

    #[derive(Debug,Clone,Serialize,Deserialize)]
    pub enum Frame {
        Init,
        Assign(Assign),
        ArtifactResponse(Exchanger<ArtifactResponse>),
    }

    impl Into<portal::outlet::Frame> for Frame {
        fn into(self) -> portal::outlet::Frame {
            match self {
                Frame::Assign(assign) => {
                    let assign = Exchanger::new(assign);
                    portal::outlet::Frame::Assign(assign)
                }
                Frame::ArtifactResponse(response) => {
                    portal::outlet::Frame::Artifact(response)
                }
                Frame::Init => {
                    portal::outlet::Frame::Init
                }
            }
        }
    }

    impl TryFrom<portal::outlet::Frame> for Frame {
        type Error = MsgErr;

        fn try_from(frame: portal::outlet::Frame) -> Result<Self, Self::Error> {
            match frame {
                portal::outlet::Frame::Assign(assign) => {
                    Ok(Frame::Assign(assign.item))
                }
                portal::outlet::Frame::Artifact(response) => {
                    Ok(Frame::ArtifactResponse(response))
                }
                portal::outlet::Frame::Init => {
                    Ok(Frame::Init)
                }

                _ => {
                    Err("no matching mechtron Frame".into())
                }
            }
        }
    }
}


#[cfg(test)]
pub mod test {

    #[test]
    pub fn test () {

    }
}