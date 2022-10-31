/*

Also known as "core_data".

This is the Rcd "core data" business layer. What was previously defined in the rcd-grpc::data_srv is intended to be
slowly moved over to a communication ambivalent layer, which is this module.

This 'core' will handle most data actions by way of the defined proto types.

*/

use crate::dbi::Dbi;

#[derive(Debug, Clone)]
pub struct Rcd {
    pub db_interface: Option<Dbi>,
}