use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// A proc macro that implements FromSql and ToSql for enums which implement
/// TryFrom and Display and want the database to store their Display value as a
/// string.
#[proc_macro_derive(SqlStringEnum)]
pub fn sql_string_enum_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        use diesel::backend::Backend;
        use diesel::deserialize;
        use diesel::serialize;
        use diesel::sql_types::Text;
        use diesel::types::{ToSql, FromSql};
        use std::io::Write;

        /// Deserialize from a database text field. Used by Diesel.
        impl<DB> FromSql<Text, DB> for #ident
        where DB: Backend,
              String: FromSql<Text, DB>
        {
            fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                #ident::try_from(String::from_sql(bytes)?)
            }
        }

        /// Serialize into a database text field. Used by Diesel.
        impl<DB> ToSql<Text, DB> for #ident
        where DB: Backend,
              str: ToSql<Text, DB>
        {
            fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
                self.to_string().as_str().to_sql(out)
            }
        }
    };
    output.into()
}
