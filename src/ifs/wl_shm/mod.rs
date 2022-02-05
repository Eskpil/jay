mod types;

use crate::client::Client;
use crate::format::FORMATS;
use crate::globals::{Global, GlobalName};
use crate::ifs::wl_shm_pool::WlShmPool;
use crate::object::{Interface, Object};
use crate::utils::buffd::MsgParser;
use std::rc::Rc;
pub use types::*;

const CREATE_POOL: u32 = 0;

const FORMAT: u32 = 0;

id!(WlShmId);

pub struct WlShmGlobal {
    name: GlobalName,
}

pub struct WlShm {
    _global: Rc<WlShmGlobal>,
    id: WlShmId,
    client: Rc<Client>,
}

impl WlShmGlobal {
    pub fn new(name: GlobalName) -> Self {
        Self { name }
    }

    fn bind_(
        self: Rc<Self>,
        id: WlShmId,
        client: &Rc<Client>,
        _version: u32,
    ) -> Result<(), WlShmError> {
        let obj = Rc::new(WlShm {
            _global: self,
            id,
            client: client.clone(),
        });
        client.add_client_obj(&obj)?;
        for format in FORMATS {
            client.event(Box::new(FormatE {
                obj: obj.clone(),
                format,
            }));
        }
        Ok(())
    }
}

impl WlShm {
    fn create_pool(&self, parser: MsgParser<'_, '_>) -> Result<(), CreatePoolError> {
        let create: CreatePool = self.client.parse(self, parser)?;
        if create.size < 0 {
            return Err(CreatePoolError::NegativeSize);
        }
        let pool = Rc::new(WlShmPool::new(
            create.id,
            &self.client,
            create.fd,
            create.size as usize,
        )?);
        self.client.add_client_obj(&pool)?;
        Ok(())
    }
}

bind!(WlShmGlobal);

impl Global for WlShmGlobal {
    fn name(&self) -> GlobalName {
        self.name
    }

    fn singleton(&self) -> bool {
        true
    }

    fn interface(&self) -> Interface {
        Interface::WlShm
    }

    fn version(&self) -> u32 {
        1
    }
}

simple_add_global!(WlShmGlobal);

object_base! {
    WlShm, WlShmError;

    CREATE_POOL => create_pool,
}

impl Object for WlShm {
    fn num_requests(&self) -> u32 {
        CREATE_POOL + 1
    }
}

simple_add_obj!(WlShm);
