use dotenv::dotenv;
use prettytable::{format, row, Table};
use viacep_rs::ViaCepClient;

fn main() {
    dotenv().ok();

    let client = ViaCepClient::new();

    match client.search("sp", "são paulo", "paulista") {
        Err(e) => eprintln!("{:?}", e),
        Ok(data) => {
            let addresses = data.unwrap();
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row![
                "State",
                "City",
                "Neighborhood",
                "Address",
                "Complement",
                "Zip"
            ]);
            for address in addresses {
                table.add_row(row![
                    address.state_initials,
                    address.city,
                    address.neighborhood,
                    address.address,
                    address.complement,
                    address.zip
                ]);
            }
            table.printstd();
        }
    }
}
