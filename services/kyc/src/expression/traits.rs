use protocol::types::Address;

pub trait ExpressionDataFeed {
    fn get_tags(&self, target_address: Address, kyc: String, tag: String) -> Vec<String>;
}
