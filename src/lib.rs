//! # PRKORM

//! The `Table` procedural macro that simplifies the creation of mysql queries for fields in your Rust structs. It comes with SELECT, INSERT, UPDATE, DELETE operations with JOINS, SUBQUERIES and other compled clauses. With this macro, you can generate these methods automatically, reducing boilerplate code and enhancing code readability.

//! ## Table of Contents
//1 - [Usage](#usage)
//! - [Examples](#examples)


//! ## Usage

//! To use the `Table` procedural macro, follow these steps:

//! 0. Add the `prkorm` crate to your `Cargo.toml`:

//!    ```toml
//!    [dependencies]
//!    prkorm = "0.1"
//!    ```

//! 1. Import the `Table` procedural macro into your Rust code:
//!     ```rust
//!     use prkorm::Table;
//!     ```

//! 2. Apply the #[derive(Table)] attribute to your struct. This will           automatically generate select(), insert(), update() and delete() methods for all the struct including but not limited to table_primary_key(), table(), select_str(), select_`field_name*`() Then you can chain functions join function, where, having, limit, order by, group by etc based on the type of query you are opting for.
//! Here is a quick example demonstrating the macro.
//!     ```rust
//!     #[derive(Table, Debug)]
//!     #[table_name("orders")]
//!     struct OrderModel {
//!         id: u32,
//!         customer_id: u32,
//!         address_id: u32,
//!         order_status: String,
//!         order_picture_url: String,
//!         created_at: String,
//!     }

//!     #[derive(Table, Debug)]
//!     #[table_name("customers")]
//!     struct Customer {
//!         id: u32,
//!         first_name: String,
//!         last_name: String,
//!     }
//!     ```

//! 3. Use the generated methods as follows:

//!     ```rust
//!         let mut select_query: String = OrderModel::select()
//!         .select_str("CONCAT_WS(' ', `first_name`, `last_name`) as username ")
//!         .select_str("address_1")
//!         .left_join_by_customer_id(Customer::table(), "id")
//!         .left_join_by_address_id("addresses", "id")
//!         .order_by_created_at_desc()
//!         .where_order_status("PENDING")
//!         .having_order_status("PENDING")
//!         .build();

//!         println!("{}",select_query);
//!         //select_query:
//!         ```sql
//!         SELECT orders.id, orders.customer_id, 
//!         orders.address_id, orders.order_status, orders.order_picture_url, orders.created_at, 
//!         CONCAT_WS(' ', `first_name`, `last_name`) as username , address_1      
//!         FROM orders
//!         LEFT JOIN customers ON customers.id = orders.customer_id
//!         LEFT JOIN addresses ON addresses.id = orders.address_id
//!         WHERE orders.order_status = 'PENDING'
//!         HAVING orders.order_status = 'PENDING'
//!         ORDER BY orders.created_at DESC
//!         ```
//!     ```

//! ## Examples
//! Here are a few examples of how to use the Table procedural macro:
//! ```rust
//! use prkorm::Table;

//!  #[derive(Table, Debug)]
//!     #[table_name("orders")]
//!     struct OrderModel {
//!         id: u32,
//!         customer_id: u32,
//!         address_id: u32,
//!         order_status: String,
//!         order_picture_url: String,
//!         created_at: String,
//!     }

//!     #[derive(Table, Debug)]
//!     #[table_name("customers")]
//!     struct Customer {
//!         id: u32,
//!         mobile_number: u64,
//!         first_name: String,
//!         last_name: String,
//!     }

//! fn main() {
//!     // SELECT QUERY
//!     let mut select_query = OrderModel::select()
//!                             .where_customer_id_in(
//!                                 Customer::select_id()
//!                                     .where_mobile_number_condition("!=","NULL")
//!                                     .build()
//!                                 )
//!                             .build();

//!     // Output
//!     println!("{}", select_query);
//!     ```sql
//!     SELECT orders.id, orders.customer_id, orders.address_id, orders.order_status, orders.order_picture_url, orders.created_at
//!     FROM orders
//!     WHERE orders.customer_id IN (SELECT customers.id
//!     FROM customers
//!     WHERE customers.mobile_number != 'NULL')
    
//!     ```
    

//!     // INSERT QUERY
//!     let insert_query = Customer::insert()
//!                         .insert_to_first_name("Prakash")
//!                         // OR "9876543210"
//!                         .insert_to_mobile_number(9876543210u64) 
//!                         .build();

//!     // Output
//!     println!("{}", insert_query);

//!     ```sql
//!     INSERT INTO customers
//!     (first_name, mobile_number) VALUES  ('Prakash', '9876543210')
//!     ```

    
//!     // UPDATE QUERY: Note => No build() in update()
//!     let update_query =  Customer::update()
//!                         .update_first_name_with_value("JOHN")
//!                         .update_last_name_with_value("WICK")
//!                         .update_where_mobile_number_eq("9876543210");


//!     // Output
//!     println!("{}", update_query);

//!     ```sql
//!     UPDATE customers SET last_name = 'WICK', first_name = 'JOHN' 
//!     WHERE mobile_number = '9876543210'


//!     // DELETE QUERY: Note =>No build() in delete()
//!     let delete_query =  Customer::delete()
//!                         .delete_where_mobile_number_eq("9876543210");


//!     // Output
//!     println!("{}", update_query);

//!     ```sql
//!     DELETE FROM customers WHERE mobile_number = '9876543210'
//!     ```



//! }



use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,  Data, DeriveInput, Fields,
    Ident, LitStr,
};



#[proc_macro_derive(Table, attributes(table_name, primary_key, table_alias))]
pub fn table_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let builder = Ident::new(&format!("{}SelectBuilder", struct_name), struct_name.span());
    let insert_builder = Ident::new(&format!("{}InsertBuilder", struct_name), struct_name.span());
    let update_builder = Ident::new(&format!("{}UpdateBuilder", struct_name), struct_name.span());
    let delete_builder = Ident::new(&format!("{}DeleteBuilder", struct_name), struct_name.span());

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => return quote! {}.into(),
        },
        _ => return quote! {}.into(),
    };

    // Extract the "table_name" attribute if present.
    let table_name_attr = ast.attrs.iter().find(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            ident == "table_name"
        } else {
            false
        }
    });
    let table_name_as_attr = ast.attrs.iter().find(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            ident == "table_alias"
        } else {
            false
        }
    });

    // Extract the value of the "table_name" attribute, if present.
    let table: Option<String> = if let Some(attr) = table_name_attr {
        if let Ok(lit) = attr.parse_args::<LitStr>() {
            Some(lit.value())
        } else {
            None
        }
    } else {
        None
    };
    // Extract the value of the "table_name" attribute, if present.
    let table_as: Option<String> = if let Some(attr) = table_name_as_attr {
        if let Ok(lit) = attr.parse_args::<LitStr>() {
            Some(lit.value())
        } else {
            table.clone()
        }
    } else {
        table.clone()
    };

    // Extract the "primary_key" attribute if present.
    let primary_key_attr = ast.attrs.iter().find(|attr| {
        if let Some(ident) = attr.path().get_ident() {
            ident == "primary_key"
        } else {
            false
        }
    });

    // Extract the value of the "table_name" attribute, if present.
    let primary_key_var = if let Some(attr) = primary_key_attr {
        if let Ok(lit) = attr.parse_args::<LitStr>() {
            lit.value()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let table_dot =  match table.clone() { Some(name) =>{
            match table_as {
                Some(ref alias) => format!("{}.", alias), 
                None =>   format!("{}.", name)
            }
        }, None => format!("")};

    let field_names = fields
        .iter()
        .map(|f| format!("{}{}",&table_dot,  f.ident.as_ref().unwrap()))
        .reduce(|acc, x| format!("{}, {}", acc, x))
        .unwrap_or(String::from("*"));

    let mut field_functions = Vec::new();
    let mut insert_functions = Vec::new();
    let mut update_functions = Vec::new();
    let mut delete_functions = Vec::new();
    let mut derived_functions = Vec::new();

    
    

    if primary_key_var.len() > 0 {
        field_functions.push(quote!(

            pub fn inner_join(mut self, table: &str,  primary_key: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
              let this_table =   &self.table_alias ;
                conditions.push(format!("\nINNER JOIN {} ON {}.{} = {}.{}", table, table, primary_key, this_table,  self.primary_key,));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn join(mut self,  table: &str, primary_key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                let this_table =   &self.table_alias ;
                conditions.push(format!("\nJOIN {} ON {}.{} = {}.{}", table, table, primary_key, this_table, self.primary_key));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn left_join(mut self, table: &str,  primary_key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                let this_table =  &self.table_alias;
                conditions.push(format!("\nLEFT JOIN {} ON {}.{} = {}.{}", table, table, primary_key,  this_table, self.primary_key));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn right_join(mut self,  table: &str, primary_key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                let this_table =  &self.table_alias;
                conditions.push(format!("\nRIGHT JOIN {} ON {}.{} = {}.{}", table, table, primary_key,  this_table, self.primary_key));
               
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn full_join(mut self, table: &str,  primary_key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                let this_table =  &self.table_alias;
                conditions.push(format!("\nRIGHT JOIN {} ON {}.{} = {}.{}", table, table, primary_key,  this_table, self.primary_key));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }

        ))
    }


    for field in fields {
       

        let field_name = field.ident.as_ref().unwrap();
        // let field_ty = &field.ty;
        let field_name_with_table =format!("{}{}", &table_dot, field_name);
        let field_name_without_table =format!("{}",field_name);
 
        let select_field_name = Ident::new(&format!("select_{}", field_name), field_name.span());
        
        let select_field_name_as = Ident::new(&format!("select_{}_as", field_name), field_name.span());
        
        let select_function_over_field_name = Ident::new(&format!("select_function_over_{}", field_name), field_name.span());
       
        let select_function_over_field_name_as = Ident::new(&format!("select_function_over_{}_as", field_name), field_name.span());

        let insert_into_col = Ident::new(&format!("insert_to_{}", field_name), field_name.span());
        
        let delete_where_col = Ident::new(&format!("delete_where_{}_eq", field_name), field_name.span());

        let update_where_col = Ident::new(&format!("update_where_{}_eq", field_name), field_name.span());
        let update_col_with_value = Ident::new(&format!("update_{}_with_value", field_name), field_name.span());

        let inner_join = Ident::new(&format!("inner_join_by_{}", field_name), field_name.span());
        let join = Ident::new(&format!("join_by_{}", field_name), field_name.span());
        let left_join = Ident::new(&format!("left_join_by_{}", field_name), field_name.span());
        let right_join = Ident::new(&format!("right_join_by_{}", field_name), field_name.span());
        let full_join = Ident::new(&format!("full_join_by_{}", field_name), field_name.span());


        let where_function_name_in = Ident::new(&format!("where_{}_in", field_name), field_name.span());
        let where_function_name = Ident::new(&format!("where_{}", field_name), field_name.span());
        let group_by_function = Ident::new(&format!("group_by_{}", field_name), field_name.span());
        let order_by_function = Ident::new(&format!("order_by_{}", field_name), field_name.span());
        let order_by_asc_function = Ident::new(&format!("order_by_{}_asc", field_name), field_name.span());
        let order_by_desc_function = Ident::new(&format!("order_by_{}_desc", field_name), field_name.span());
        let having_function = Ident::new(&format!("having_{}", field_name), field_name.span());
        let where_function_operator_name = Ident::new(
            &format!("where_{}_condition", field_name),
            field_name.span(),
        );

        delete_functions.push(quote! {
            pub fn #delete_where_col(mut self, value: impl ToString) -> String {
                format!("DELETE FROM {} WHERE {} = '{}'", &self.table, #field_name_without_table, value.to_string())
            }
        });

        update_functions.push(quote! {
              pub fn #update_where_col(mut self, value: impl ToString) -> String {
                let mut set_values = String::new();
                for (i, (k, v)) in self.selected.clone().into_iter().enumerate() {
                    set_values = format!("{}{} = '{}'", set_values, k.clone(), v.clone());
                    if i + 1 != self.selected.len() {
                        set_values = format!("{}, ", set_values);
                    }
                }
                format!("UPDATE {} SET {} \nWHERE {} = '{}'", &self.table, set_values.clone(),  #field_name_without_table.clone(), value.to_string())
              }  

              pub fn #update_col_with_value(mut self, value: impl ToString) -> Self {
                let mut selected =  self.selected.clone();
                 selected.entry(#field_name_without_table.to_string()).or_insert(value.to_string());
                Self {
                    selected: selected,
                    ..self
                }
              }  
            }
        );

        insert_functions.push(quote! {

            pub fn #insert_into_col(mut self, value : impl ToString) -> Self {
                let mut selected =  self.selected.clone();
                 selected.entry(#field_name_without_table.to_string()).or_insert(vec![value.to_string()]);
                Self {
                    selected: selected,
                    ..self
                }
            }

            pub fn #order_by_function(mut self, order : &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} {}",#field_name_with_table, order));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn #order_by_asc_function(mut self) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} ASC",#field_name_with_table));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn #order_by_desc_function(mut self) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} DESC",#field_name_with_table));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

        });

        derived_functions.push(quote! {


            pub fn #select_field_name() -> #builder {
              
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    order_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}", #field_name_with_table),
                }
            }

            pub fn #select_function_over_field_name( function: &str ) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    order_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}({})", function.to_uppercase(),  #field_name_with_table),
                }
            }

           

            pub fn #select_function_over_field_name_as(mut self, function: &str , alias: &str ) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    order_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}({}) AS {}", function.to_uppercase(),  #field_name_with_table, alias),
                }
            }
        });
        
        field_functions.push(quote! {

            pub fn #select_field_name(mut self) -> Self {
                Self {
                    selected: format!("{}, {}", self.selected, #field_name_with_table),
                    ..self
                }
            }
            
            pub fn #select_field_name_as(mut self, alias: &str) -> Self {
                Self {
                    selected: format!("{}, ({}) AS {}", self.selected, #field_name_with_table, alias),
                    ..self
                }
            }

            pub fn #select_function_over_field_name(mut self, function: &str ) -> Self {
                Self {
                    selected: format!("{}, {}({})", self.selected, function.to_uppercase() ,#field_name_with_table ),
                    ..self
                }
            }

            pub fn #select_function_over_field_name_as(mut self, function: &str , alias: &str ) -> Self {
                Self {
                    selected: format!("{}, {}({}) AS {}", self.selected, function.to_uppercase() ,#field_name_with_table , alias),
                    ..self
                }
            }


            pub fn #inner_join(mut self, table: &str,  key: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\nINNER JOIN {} ON {}.{} = {}", table,table, key, #field_name_with_table));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn #join(mut self, table: &str,  key: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\nJOIN {} ON {}.{} = {}", table,table, key, #field_name_with_table));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn #left_join(mut self,  table: &str, key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\nLEFT JOIN {} ON {}.{} = {}", table,table, key, #field_name_with_table));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn #right_join(mut self, table: &str, key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\nRIGHT JOIN {} ON {}.{} = {}", table,table, key, #field_name_with_table));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            pub fn #full_join(mut self,  table: &str, key: &str,) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\nFULL JOIN {} ON {}.{} = {}", table,table, key, #field_name_with_table));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }
            
            pub fn #order_by_function(mut self, order : &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} {}",#field_name_with_table, order));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn #order_by_asc_function(mut self) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} ASC",#field_name_with_table));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn #order_by_desc_function(mut self) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{} DESC",#field_name_with_table));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }
            
            pub fn #group_by_function(mut self) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.group_by);
                conditions.push(format!("{}",#field_name_with_table));
                Self {
                    group_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn #having_function(mut self, #field_name: impl ToString) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.having);
                conditions.push(format!("{} = '{}'",#field_name_with_table, #field_name.to_string() ));
                Self {
                    having: conditions.clone(), 
                    ..self
                }
            }
            pub fn #where_function_name_in(mut self, where_in: impl ToString) -> Self {
                let where_in = where_in.to_string();
                if where_in.trim().is_empty() {
                  return  self;
                }
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.where_conditions);
                conditions.push(format!("{} IN ({})", #field_name_with_table,  where_in ));
                Self {
                    where_conditions: conditions.clone(), 
                    ..self
                }
            }
            pub fn #where_function_name(mut self, #field_name:impl ToString) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.where_conditions);
                conditions.push(format!("{} = '{}'",#field_name_with_table,  #field_name.to_string() ));
                Self {
                    where_conditions: conditions.clone(), 
                    ..self
                }
            }
            pub fn #where_function_operator_name(mut self, operator: &str,  #field_name: impl ToString,) -> Self  {
                // self.#field_name = update_with;
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.where_conditions);
                conditions.push(format!("{} {} '{}'",#field_name_with_table, operator, #field_name.to_string() ));
                Self {
                    where_conditions: conditions.clone(), 
                    ..self
                }
            }
        });
    }



    // Generate the struct and its associated functions.
    let gen = quote! {
       

        #[derive(Debug, Clone, Default)]
        pub struct #delete_builder {
            table: String,
        }

        impl #delete_builder {
            pub fn delete_where_str(mut self, raw: &str) -> String {
                format!("DELETE FROM {} WHERE {}", &self.table, raw)
            } 

            #(#delete_functions)*
        }


        #[derive(Debug, Clone, Default)]
        pub struct #update_builder {
            selected: std::collections::HashMap<String, String>,
            table: String,
        }

        impl #update_builder {

            pub fn where_str(mut self, where_condition: &str) -> String {
                let mut set_values = String::new();
                for (i, (k, v)) in self.selected.clone().into_iter().enumerate() {
                    set_values = format!("{}{} = '{}'", set_values, k.clone(), v.clone());
                    if i + 1 != self.selected.len() {
                        set_values = format!("{}, ", set_values);
                    }
                }
                format!("UPDATE {} SET {} WHERE {}", &self.table, set_values, where_condition)
            }

            #(#update_functions)*

        }

        #[derive(Debug, Clone, Default)]
        pub struct #insert_builder {
            selected: std::collections::HashMap<String, Vec<String>>,
            table: String,
            limit: Option<u32>,
            order_by: Vec<String>,
        }

        impl  #insert_builder {

            pub fn limit(mut self, limit: u32) -> Self {
                Self {
                    limit: Some(limit), 
                    ..self
                }
            }

            #(#insert_functions)*

                     pub fn build(self) -> String {
                let mut keys = String::new();
                let mut values = String::new();
                for (i, (k, v)) in self.selected.clone().into_iter().enumerate() {
                    keys = format!("{}{}", keys, k.clone());
                    if (i + 1 != self.selected.len()) {
                        keys = format!("{}, ", keys);
                    }
                }
                         let mut inputs = Vec::new();
                 let mut results = Vec::new();

                 for (k, v) in self.selected.clone().into_iter() {
                 inputs.push(v);
                 }
                 for i in 0..inputs.first().unwrap().len() {
                 let mut data = Vec::new();
                 for j in 0..inputs.len() {
                     data.push(inputs[j][i].clone());
                 }
                 results.push(data);
                 }
                 for i in 0..results.len() {
                 let item = results[i].clone();
                 let mut value = String::new();
                 for j in 0..item.len() {
            value = format!("{}'{}'", value, item[j]);
            if j + 1 != item.len() {
                value = format!("{}, ", value);
                     }
                 }
                    values = format!("{} ({})", values, value);
                    if i + 1 != results.len() {
            values = format!("{},", values);
                    }
                    }
                format!("INSERT INTO {}\n({}) VALUES {}", &self.table, keys, values)
            }



        }

        #[derive(Debug, Clone)]
        pub struct #builder {
            selected: String,
            joins: Vec<String>,
            primary_key: String,
            table: String,
            table_alias: String,
            limit: Option<u32>,
            where_conditions: Vec<String>,
            group_by: Vec<String>,
            order_by: Vec<String>,
            having: Vec<String>,
        }

        impl #builder {

            pub fn join_str(mut self, join: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.joins);
                conditions.push(format!("\n{}", join));
                Self {
                    joins: conditions.clone(),
                    ..self
                }
            }

            pub fn having_str(mut self, having: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                    conditions.append(&mut self.having);
                    conditions.push(format!("{}", having ));
                    Self {
                        
                        having: conditions.clone(), 
                        ..self
                    }
            }
            pub fn where_str(mut self, where_query: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                    conditions.append(&mut self.where_conditions);
                    conditions.push(format!("{}", where_query ));
                    Self {
                        
                        where_conditions: conditions.clone(), 
                        ..self
                    }
            }
            pub fn group_by_str(mut self, group_by: &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                    conditions.append(&mut self.group_by);
                    conditions.push(format!("{}", group_by ));
                    Self {
                        
                        group_by: conditions.clone(), 
                        ..self
                    }
            }

            pub fn order_by_str(mut self, order : &str) -> Self {
                let mut conditions: Vec<String> = Vec::new();
                conditions.append(&mut self.order_by);
                conditions.push(format!("{}", order));
                Self {
                    order_by: conditions.clone(), 
                    ..self
                }
            }

            pub fn select_function_as(mut self, function: &str , over: &str , alias: &str ) -> Self {
                Self {
                    selected: format!("{}, {}({}) AS {}", self.selected, function.to_uppercase() ,over, alias),
                    ..self
                }
            }

            pub fn select_as(mut self, selection: &str, alias: &str) -> Self {
                Self {
                    selected: format!("{}, ({}) AS {}", self.selected, selection, alias),
                    ..self
                }
            }

            pub fn select_str(mut self, select: &str) -> Self {
                Self {
                    selected: format!("{}, {}", self.selected, select),
                    ..self
                }
            }

            pub fn limit(mut self, limit: u32) -> Self {
                Self {
                    limit: Some(limit), 
                    ..self
                }
            }

            #(#field_functions)*


            pub fn build(&self) -> String {
                let limit = match self.limit {
                    Some(limit) => format!(" \nLIMIT {}", limit), 
                    None => String::new()
                };
               
                    let mut where_query = String::new();
                    for i in 0..self.where_conditions.len() {
                        if(i ==0) {
                            where_query = format!(" \nWHERE");
                        }
                        where_query = format!("{} {}", where_query, self.where_conditions[i].clone());
                        if (i + 1 != self.where_conditions.len()) {
                            where_query = format!("{} {}", where_query, "AND");
                        }
                    }
                    let mut joins = String::new();
                    for i in 0..self.joins.len() {
                        if(i ==0) {
                            joins = format!(" ");
                        }
                        joins = format!("{} {} ", joins, self.joins[i].clone());
                        
                    }
                    let mut group_by = String::new();
                    for i in 0..self.group_by.len() {
                        if(i ==0) {
                            group_by = format!(" \nGROUP BY");
                        }
                        group_by = format!("{} {}", group_by, self.group_by[i].clone());
                        if (i + 1 != self.group_by.len()) {
                            group_by = format!("{},", group_by);
                        }
                    }
                    let mut order_by = String::new();
                    for i in 0..self.order_by.len() {
                        if(i ==0) {
                            order_by = format!(" \nORDER BY");
                        }
                        order_by = format!("{} {}", order_by, self.order_by[i].clone());
                        if (i + 1 != self.order_by.len()) {
                            order_by = format!("{},", order_by);
                        }
                    }
                    let mut having = String::new();
                    for i in 0..self.having.len() {
                        if(i ==0) {
                            having = format!(" \nHAVING");
                        }
                        having = format!("{} {}", having, self.having[i].clone());
                        if (i + 1 != self.having.len()) {
                            having = format!("{} AND", having);
                        }
                    }
                    let this_table =  match &self.table_alias == &self.table  {
                        true => "", 
                        false => &self.table_alias
                    };
                    format!("SELECT {} \nFROM {} {}{}{}{}{}{}{}", self.selected, self.table ,this_table ,joins, where_query, group_by, having,order_by, limit)
            }
        }

        impl #struct_name {

            pub fn delete() -> #delete_builder {
                #delete_builder {
                    table: #table.into()
                }
            }

            pub fn update() -> #update_builder {
                #update_builder {
                    table: #table.into(), 
                    ..#update_builder::default()
                }
            }

            pub fn insert() -> #insert_builder {
                #insert_builder {
                    table: #table.into(),
                    ..#insert_builder::default()
                }
            }

            pub fn select() -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    order_by: Vec::new(),
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}", #field_names),
                }
            }

            pub fn select_function_over_field_name( function: &str, over: &str ) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    order_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}({})", function.to_uppercase(),  over),
                }
            }
            pub fn select_function_over_field_name_as( function: &str, over: &str, alias: &str ) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    order_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}({}) AS {}", function.to_uppercase(),  over, alias),
                }
            }

            pub fn select_str(select: &str) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    order_by: Vec::new(),
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("{}", select),
                }
            }

            pub fn select_str_as(select: &str, alias: &str) -> #builder {
                #builder {
                    primary_key: Self::table_primary_key(),
                    limit: None,
                    order_by: Vec::new(),
                    joins: Vec::new(),
                    where_conditions: Vec::new(),
                    group_by: Vec::new(),
                    having: Vec::new(),
                    table: #table.into(),
                    table_alias: #table_as.into(),
                    selected: format!("({}) AS {}", select, alias),
                }
            }

            #(#derived_functions)*

            pub fn table() -> &'static str {
                #table
            } 
            pub fn table_name(&self) -> &'static str {
                #table
            }
            
            pub fn table_primary_key() -> String {
                format!("{}", #primary_key_var)
            }
        }


    };
    gen.into()
}