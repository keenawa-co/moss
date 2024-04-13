use async_graphql::SimpleObject;
use nanoid::nanoid;

const ID_SIZE: usize = 20;
const CHAR_SET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[cfg(feature = "gql")]
#[derive(SimpleObject)]
#[revisioned(revision = 1)]
pub struct Thing {
    pub id: String,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            id: nanoid!(ID_SIZE, &CHAR_SET),
        }
    }
}

impl From<String> for Thing {
    fn from(value: String) -> Self {
        Self { id: value }
    }
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
