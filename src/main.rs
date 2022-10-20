/*
    potato_field - main server for routing connections for the BW API
    Copyright (C) 2022  Bailey Danyluk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
extern crate warp;
use warp::Filter;
use tokio::sync::oneshot;

pub mod routes;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    let (addr, server) = warp::serve(routes::route()).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async { println!("inside"); rx.await.ok(); });

    tokio::task::spawn(server);

    loop {}
    let _ = tx.send(());
}
