use serde::{ Serialize, Deserialize };

// impl Status {
//     pub fn code(&self) -> i32 {
//         self.code
//     }
//     pub fn message(&self) -> &str {
//         &self.message
//     }
//     pub fn detail(&self) -> &Option<String> {
//         &self.detail
//     }
// }

// impl ExistingKeys {
//     pub fn status(&self) -> &Status {
//         &self.status
//     }
//     pub fn session_expires_in_seconds(&self) -> i32 {
//         self.session_expires_in_seconds
//     }
//     pub fn keys(&self) -> &Vec<Key> {
//         &self.keys
//     }

//     pub fn remove_all_invalid_keys(&mut self, ip: String) {
//         for i in 0..self.keys.len() {
//             if !self.keys.get(i).unwrap().cidr_ranges.contains(&ip) {
//                 self.keys.remove(i);
//             }
//         }
//     }
// }

// impl Key {
//     pub fn id(&self) -> &str {
//         &self.id
//     }
//     pub fn developer_id(&self) -> &str {
//         &self.developer_id
//     }
//     pub fn tier(&self) -> &str {
//         &self.tier
//     }
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//     pub fn description(&self) -> &str {
//         &self.description
//     }
//     pub fn origins(&self) -> &Option<String> {
//         &self.origins
//     }
//     pub fn scopes(&self) -> &Vec<Scope> {
//         &self.scopes
//     }
//     pub fn cidr_ranges(&self) -> &Vec<String> {
//         &self.cidr_ranges
//     }
//     pub fn valid_until(&self) -> &Option<String> {
//         &self.valid_until
//     }
//     pub fn key(&self) -> &str {
//         &self.key
//     }
// }