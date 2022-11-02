//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use crate::*;
use zenoh_buffers::{
    reader::{DidntRead, Reader},
    writer::{DidntWrite, Writer},
};
use zenoh_protocol::{
    common::imsg,
    core::{Locator, WhatAmI, ZInt, ZenohId},
    scouting::Hello,
    transport::tmsg,
};

impl<W> WCodec<&mut W, &Hello> for Zenoh060
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    fn write(self, writer: &mut W, x: &Hello) -> Self::Output {
        // Header
        let mut header = tmsg::id::HELLO;
        if x.zid.is_some() {
            header |= tmsg::flag::I
        }
        if x.whatami != WhatAmI::Router {
            header |= tmsg::flag::W;
        }
        if !x.locators.is_empty() {
            header |= tmsg::flag::L;
        }
        zcwrite!(self, writer, header)?;

        // Body
        if let Some(zid) = x.zid.as_ref() {
            zcwrite!(self, writer, zid)?;
        }
        if x.whatami != WhatAmI::Router {
            let wai: ZInt = x.whatami.into();
            zcwrite!(self, writer, wai)?;
        }
        if !x.locators.is_empty() {
            zcwrite!(self, writer, x.locators.as_slice())?;
        }
        Ok(())
    }
}

impl<R> RCodec<&mut R, Hello> for Zenoh060
where
    R: Reader,
{
    type Error = DidntRead;

    fn read(self, reader: &mut R) -> Result<Hello, Self::Error> {
        let codec = Zenoh060RCodec {
            header: self.read(&mut *reader)?,
            ..Default::default()
        };
        codec.read(reader)
    }
}

impl<R> RCodec<&mut R, Hello> for Zenoh060RCodec
where
    R: Reader,
{
    type Error = DidntRead;

    fn read(self, reader: &mut R) -> Result<Hello, Self::Error> {
        if imsg::mid(self.header) != imsg::id::HELLO {
            return Err(DidntRead);
        }

        let zid = if imsg::has_flag(self.header, tmsg::flag::I) {
            let zid: ZenohId = zcread!(self.codec, reader)?;
            Some(zid)
        } else {
            None
        };
        let whatami = if imsg::has_flag(self.header, tmsg::flag::W) {
            let wai: ZInt = zcread!(self.codec, reader)?;
            WhatAmI::try_from(wai).ok_or(DidntRead)?
        } else {
            WhatAmI::Router
        };
        let locators = if imsg::has_flag(self.header, tmsg::flag::L) {
            let locs: Vec<Locator> = zcread!(self.codec, reader)?;
            locs
        } else {
            vec![]
        };

        Ok(Hello {
            zid,
            whatami,
            locators,
        })
    }
}
