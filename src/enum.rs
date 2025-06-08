// Enums give you a way of saying a value is one of a possible set of values.
fn enum_l(){
    //it means ip address can be or v4 or v6
    enum IpAddress {
        V4(String),
        V6(String),
    }

    struct IpAddr{
        kind: IpAddress,
        address: String,
    }

    let home1 = IpAddr{
        kind: IpAddress::V4,
        address: String::from("127.0.0.1"),
    };

    let home2 = IpAddr{
        kind: IpAddress::V6,
        address: String::from("::1"),
    } ;

    let four = IpAddress::V4;
    let six = IpAddress::V6;

    fn route(ip_kind: IpAddress){
//         println!(four.address);
    }
    route(IpAddress::V4);
    route(IpAddress::V6);
}