fn main() {
    enum IpAddrKing {
        V4(String),
        V6(String),
    }
    let _four = IpAddrKing::V4;
    let _six = IpAddrKing::V6;

    fn route(_ip_kind: IpAddrKing){

    }


    route(_ip_kind: IpAddrKing::V4);
    route(_ip_kind: IpAddrKing::V6);

}
