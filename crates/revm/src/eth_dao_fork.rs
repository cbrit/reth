//! DAO FOrk related constants from [EIP-779](https://eips.ethereum.org/EIPS/eip-779).
//! It happened on Ethereum block 1_920_000
use reth_primitives::{hex_literal::hex, H160};

/// Dao hardfork beneficiary that received ether from accounts from DAO and DAO creator children.
pub static DAO_HARDFORK_BENEFICIARY: H160 = H160(hex!("bf4ed7b27f1d666546e30d74d50d173d20bca754"));

/// DAO hardfork account that ether was taken and added to beneficiary
pub static DAO_HARDKFORK_ACCOUNTS: [H160; 116] = [
    H160(hex!("d4fe7bc31cedb7bfb8a345f31e668033056b2728")),
    H160(hex!("b3fb0e5aba0e20e5c49d252dfd30e102b171a425")),
    H160(hex!("2c19c7f9ae8b751e37aeb2d93a699722395ae18f")),
    H160(hex!("ecd135fa4f61a655311e86238c92adcd779555d2")),
    H160(hex!("1975bd06d486162d5dc297798dfc41edd5d160a7")),
    H160(hex!("a3acf3a1e16b1d7c315e23510fdd7847b48234f6")),
    H160(hex!("319f70bab6845585f412ec7724b744fec6095c85")),
    H160(hex!("06706dd3f2c9abf0a21ddcc6941d9b86f0596936")),
    H160(hex!("5c8536898fbb74fc7445814902fd08422eac56d0")),
    H160(hex!("6966ab0d485353095148a2155858910e0965b6f9")),
    H160(hex!("779543a0491a837ca36ce8c635d6154e3c4911a6")),
    H160(hex!("2a5ed960395e2a49b1c758cef4aa15213cfd874c")),
    H160(hex!("5c6e67ccd5849c0d29219c4f95f1a7a93b3f5dc5")),
    H160(hex!("9c50426be05db97f5d64fc54bf89eff947f0a321")),
    H160(hex!("200450f06520bdd6c527622a273333384d870efb")),
    H160(hex!("be8539bfe837b67d1282b2b1d61c3f723966f049")),
    H160(hex!("6b0c4d41ba9ab8d8cfb5d379c69a612f2ced8ecb")),
    H160(hex!("f1385fb24aad0cd7432824085e42aff90886fef5")),
    H160(hex!("d1ac8b1ef1b69ff51d1d401a476e7e612414f091")),
    H160(hex!("8163e7fb499e90f8544ea62bbf80d21cd26d9efd")),
    H160(hex!("51e0ddd9998364a2eb38588679f0d2c42653e4a6")),
    H160(hex!("627a0a960c079c21c34f7612d5d230e01b4ad4c7")),
    H160(hex!("f0b1aa0eb660754448a7937c022e30aa692fe0c5")),
    H160(hex!("24c4d950dfd4dd1902bbed3508144a54542bba94")),
    H160(hex!("9f27daea7aca0aa0446220b98d028715e3bc803d")),
    H160(hex!("a5dc5acd6a7968a4554d89d65e59b7fd3bff0f90")),
    H160(hex!("d9aef3a1e38a39c16b31d1ace71bca8ef58d315b")),
    H160(hex!("63ed5a272de2f6d968408b4acb9024f4cc208ebf")),
    H160(hex!("6f6704e5a10332af6672e50b3d9754dc460dfa4d")),
    H160(hex!("77ca7b50b6cd7e2f3fa008e24ab793fd56cb15f6")),
    H160(hex!("492ea3bb0f3315521c31f273e565b868fc090f17")),
    H160(hex!("0ff30d6de14a8224aa97b78aea5388d1c51c1f00")),
    H160(hex!("9ea779f907f0b315b364b0cfc39a0fde5b02a416")),
    H160(hex!("ceaeb481747ca6c540a000c1f3641f8cef161fa7")),
    H160(hex!("cc34673c6c40e791051898567a1222daf90be287")),
    H160(hex!("579a80d909f346fbfb1189493f521d7f48d52238")),
    H160(hex!("e308bd1ac5fda103967359b2712dd89deffb7973")),
    H160(hex!("4cb31628079fb14e4bc3cd5e30c2f7489b00960c")),
    H160(hex!("ac1ecab32727358dba8962a0f3b261731aad9723")),
    H160(hex!("4fd6ace747f06ece9c49699c7cabc62d02211f75")),
    H160(hex!("440c59b325d2997a134c2c7c60a8c61611212bad")),
    H160(hex!("4486a3d68fac6967006d7a517b889fd3f98c102b")),
    H160(hex!("9c15b54878ba618f494b38f0ae7443db6af648ba")),
    H160(hex!("27b137a85656544b1ccb5a0f2e561a5703c6a68f")),
    H160(hex!("21c7fdb9ed8d291d79ffd82eb2c4356ec0d81241")),
    H160(hex!("23b75c2f6791eef49c69684db4c6c1f93bf49a50")),
    H160(hex!("1ca6abd14d30affe533b24d7a21bff4c2d5e1f3b")),
    H160(hex!("b9637156d330c0d605a791f1c31ba5890582fe1c")),
    H160(hex!("6131c42fa982e56929107413a9d526fd99405560")),
    H160(hex!("1591fc0f688c81fbeb17f5426a162a7024d430c2")),
    H160(hex!("542a9515200d14b68e934e9830d91645a980dd7a")),
    H160(hex!("c4bbd073882dd2add2424cf47d35213405b01324")),
    H160(hex!("782495b7b3355efb2833d56ecb34dc22ad7dfcc4")),
    H160(hex!("58b95c9a9d5d26825e70a82b6adb139d3fd829eb")),
    H160(hex!("3ba4d81db016dc2890c81f3acec2454bff5aada5")),
    H160(hex!("b52042c8ca3f8aa246fa79c3feaa3d959347c0ab")),
    H160(hex!("e4ae1efdfc53b73893af49113d8694a057b9c0d1")),
    H160(hex!("3c02a7bc0391e86d91b7d144e61c2c01a25a79c5")),
    H160(hex!("0737a6b837f97f46ebade41b9bc3e1c509c85c53")),
    H160(hex!("97f43a37f595ab5dd318fb46e7a155eae057317a")),
    H160(hex!("52c5317c848ba20c7504cb2c8052abd1fde29d03")),
    H160(hex!("4863226780fe7c0356454236d3b1c8792785748d")),
    H160(hex!("5d2b2e6fcbe3b11d26b525e085ff818dae332479")),
    H160(hex!("5f9f3392e9f62f63b8eac0beb55541fc8627f42c")),
    H160(hex!("057b56736d32b86616a10f619859c6cd6f59092a")),
    H160(hex!("9aa008f65de0b923a2a4f02012ad034a5e2e2192")),
    H160(hex!("304a554a310c7e546dfe434669c62820b7d83490")),
    H160(hex!("914d1b8b43e92723e64fd0a06f5bdb8dd9b10c79")),
    H160(hex!("4deb0033bb26bc534b197e61d19e0733e5679784")),
    H160(hex!("07f5c1e1bc2c93e0402f23341973a0e043f7bf8a")),
    H160(hex!("35a051a0010aba705c9008d7a7eff6fb88f6ea7b")),
    H160(hex!("4fa802324e929786dbda3b8820dc7834e9134a2a")),
    H160(hex!("9da397b9e80755301a3b32173283a91c0ef6c87e")),
    H160(hex!("8d9edb3054ce5c5774a420ac37ebae0ac02343c6")),
    H160(hex!("0101f3be8ebb4bbd39a2e3b9a3639d4259832fd9")),
    H160(hex!("5dc28b15dffed94048d73806ce4b7a4612a1d48f")),
    H160(hex!("bcf899e6c7d9d5a215ab1e3444c86806fa854c76")),
    H160(hex!("12e626b0eebfe86a56d633b9864e389b45dcb260")),
    H160(hex!("a2f1ccba9395d7fcb155bba8bc92db9bafaeade7")),
    H160(hex!("ec8e57756626fdc07c63ad2eafbd28d08e7b0ca5")),
    H160(hex!("d164b088bd9108b60d0ca3751da4bceb207b0782")),
    H160(hex!("6231b6d0d5e77fe001c2a460bd9584fee60d409b")),
    H160(hex!("1cba23d343a983e9b5cfd19496b9a9701ada385f")),
    H160(hex!("a82f360a8d3455c5c41366975bde739c37bfeb8a")),
    H160(hex!("9fcd2deaff372a39cc679d5c5e4de7bafb0b1339")),
    H160(hex!("005f5cee7a43331d5a3d3eec71305925a62f34b6")),
    H160(hex!("0e0da70933f4c7849fc0d203f5d1d43b9ae4532d")),
    H160(hex!("d131637d5275fd1a68a3200f4ad25c71a2a9522e")),
    H160(hex!("bc07118b9ac290e4622f5e77a0853539789effbe")),
    H160(hex!("47e7aa56d6bdf3f36be34619660de61275420af8")),
    H160(hex!("acd87e28b0c9d1254e868b81cba4cc20d9a32225")),
    H160(hex!("adf80daec7ba8dcf15392f1ac611fff65d94f880")),
    H160(hex!("5524c55fb03cf21f549444ccbecb664d0acad706")),
    H160(hex!("40b803a9abce16f50f36a77ba41180eb90023925")),
    H160(hex!("fe24cdd8648121a43a7c86d289be4dd2951ed49f")),
    H160(hex!("17802f43a0137c506ba92291391a8a8f207f487d")),
    H160(hex!("253488078a4edf4d6f42f113d1e62836a942cf1a")),
    H160(hex!("86af3e9626fce1957c82e88cbf04ddf3a2ed7915")),
    H160(hex!("b136707642a4ea12fb4bae820f03d2562ebff487")),
    H160(hex!("dbe9b615a3ae8709af8b93336ce9b477e4ac0940")),
    H160(hex!("f14c14075d6c4ed84b86798af0956deef67365b5")),
    H160(hex!("ca544e5c4687d109611d0f8f928b53a25af72448")),
    H160(hex!("aeeb8ff27288bdabc0fa5ebb731b6f409507516c")),
    H160(hex!("cbb9d3703e651b0d496cdefb8b92c25aeb2171f7")),
    H160(hex!("6d87578288b6cb5549d5076a207456a1f6a63dc0")),
    H160(hex!("b2c6f0dfbb716ac562e2d85d6cb2f8d5ee87603e")),
    H160(hex!("accc230e8a6e5be9160b8cdf2864dd2a001c28b6")),
    H160(hex!("2b3455ec7fedf16e646268bf88846bd7a2319bb2")),
    H160(hex!("4613f3bca5c44ea06337a9e439fbc6d42e501d0a")),
    H160(hex!("d343b217de44030afaa275f54d31a9317c7f441e")),
    H160(hex!("84ef4b2357079cd7a7c69fd7a37cd0609a679106")),
    H160(hex!("da2fef9e4a3230988ff17df2165440f37e8b1708")),
    H160(hex!("f4c64518ea10f995918a454158c6b61407ea345c")),
    H160(hex!("7602b46df5390e432ef1c307d4f2c9ff6d65cc97")),
    H160(hex!("bb9bc244d798123fde783fcc1c72d3bb8c189413")),
    H160(hex!("807640a13483f8ac783c557fcdf27be11ea4ac7a")),
];
