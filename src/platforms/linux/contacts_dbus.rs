//! `contacts_dbus` gets contacts from Evolution's EDS interface, for inserting into CoDi over UART.

#![allow(dead_code)]

use std::io::BufReader;

use anyhow::Result;
use thiserror::Error;
use zbus::blocking::{Connection, Proxy};
use zbus::Error as ZbusError;

#[derive(Clone, Debug)]
pub(crate) struct CoDiContactNumber {
    phone_type: String,
    number: String,
}

impl CoDiContactNumber {
    pub(crate) fn get_phone_type(&self) -> String {
        self.phone_type.to_owned()
    }

    pub(crate) fn get_phone_number(&self) -> String {
        self.number.to_owned()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct CoDiContact {
    pub(crate) name: String,
    pub(crate) phone: Vec<CoDiContactNumber>,
}

impl CoDiContact {
    pub(crate) fn get_contact_name(&self) -> String {
        self.name.to_owned()
    }

    pub(crate) fn get_numbers(&self) -> Vec<CoDiContactNumber> {
        self.phone.to_owned()
    }
}

#[derive(Debug, Error)]
pub(crate) enum DbusContactsError {
    #[error("Failed to connect to the D-Bus User Session bus.")]
    SessionBusConnectFailure(#[source] ZbusError),
    #[error("Error retrieving D-Bus activatable interfaces.")]
    ListInterfacesFailure(#[source] ZbusError),
    #[error("EDS interface missing.")]
    EdsInterfaceMissingFailure,
    #[error("Error searching for the EDS interface.")]
    EdsInterfaceSearchFailure(#[source] ZbusError),
    #[error("Error opening EDS address book.")]
    AddressBookOpenFailure(#[source] ZbusError),
    #[error("Error getting `Vec<T>` of contacts")]
    GetContactsFailure(#[source] ZbusError),
    #[error("Error getting single contact.")]
    GetContactFailure(#[source] ZbusError),
}

pub(crate) type CoDiContacts = Vec<CoDiContact>;
pub(crate) type CoDiDbusContactsResult =
Result<CoDiContacts, DbusContactsError>;

pub(crate) fn get_dbus_contacts() -> CoDiDbusContactsResult {
    trace!("Get connection to session bus...");

    let bus = Connection::session()
        .map_err(|e| DbusContactsError::SessionBusConnectFailure(e))?;

    let all_ifaces = Proxy::new(&bus, "org.freedesktop.DBus",
                                "/",
                                "org.freedesktop.DBus")
        .map_err(|e| DbusContactsError::EdsInterfaceSearchFailure(e))?;

    let dbus_all_names: Vec<String> = all_ifaces.call("ListActivatableNames",
                                                      &())
        .map_err(|e| DbusContactsError::EdsInterfaceSearchFailure(e))?;

    if !dbus_all_names.contains(&String::from("org.gnome.evolution.dataserver.AddressBook")) {
        return Err(DbusContactsError::EdsInterfaceMissingFailure);
    }

    let address_book_bus = "org.gnome.evolution.dataserver.AddressBook";

    let mut proxy = Proxy::new(&bus, address_book_bus,
                               "/org/gnome/evolution/dataserver/AddressBookFactory",
                               "org.gnome.evolution.dataserver.AddressBookFactory")
        .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    let (obj_path, eds_bus): (String, String) = proxy.call("OpenAddressBook",
                                                           &("system-address-book", ))
        .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    proxy = Proxy::new(&bus, eds_bus, obj_path,
                       "org.gnome.evolution.dataserver.AddressBook")
        .map_err(|e| DbusContactsError::AddressBookOpenFailure(e))?;

    let (contacts, ): (Vec<String>, ) = proxy.call(
        "GetContactListUids",
        &("", ), )
        .map_err(|e| DbusContactsError::GetContactsFailure(e))?;

    let mut codi_contacts: Vec<CoDiContact> = Vec::new();

    for contact_id in contacts {
        let (contact, ): (String, ) = proxy.call(
            "GetContact",
            &(contact_id, ), )
            .map_err(|e| DbusContactsError::GetContactFailure(e))?;

        let reader = ical::VcardParser::new(BufReader::new(contact.as_bytes()));

        let mut contact_name = String::new();
        let mut contact_numbers: Vec<CoDiContactNumber> = Vec::new();
        'vcardloop: for line in reader {
            println!("{:?}", line);

            let line = match line {
                Ok(res) => res,
                _ => continue 'vcardloop,
            };

            for prop in line.clone().properties {
                let params = match prop.clone().params {
                    Some(res) => res,
                    _ => continue 'vcardloop,
                };
                let value = match prop.clone().value {
                    Some(res) => res,
                    _ => continue 'vcardloop,
                };

                if prop.name == "FN" {
                    contact_name = match prop.clone().value {
                        Some(res) => res.clone(),
                        _ => continue 'vcardloop,
                    };
                }

                if prop.name == "TEL" {
                    for entry in params {
                        if &entry.0 == "TYPE" {
                            let phone_type = entry.1[0].clone();
                            let phone_number = value.clone();

                            contact_numbers.push(CoDiContactNumber {
                                phone_type: phone_type.to_string(),
                                number: phone_number.to_string(),
                            });
                        }
                    }
                }

                break 'vcardloop;
            }
        }

        if contact_name.is_empty() {
            continue;
        }

        if contact_numbers.is_empty() {
            continue;
        }

        let codi_contact = CoDiContact {
            name: contact_name,
            phone: contact_numbers,
        };

        codi_contacts.push(codi_contact);
    }

    Ok(codi_contacts)
}
