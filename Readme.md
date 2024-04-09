# Table Procedural Macro

The `Table` procedural macro that simplifies the creation of SQL queries based on fields in your Rust structs. It comes with SELECT, INSERT, UPDATE, DELETE operations with JOINS, SUBQUERIES and other compled clauses. With this macro, you can generate these methods automatically, reducing boilerplate code and enhancing code readability.

## Table of Contents
- [Usage](#usage)
- [Examples](#examples)


## Usage

To use the `Table` procedural macro, follow these steps:

0. Add the `prkorm` crate to your `Cargo.toml`:

   ```toml
   [dependencies]
   prkorm = "0.1"
   ```

1. Import the `Table` procedural macro into your Rust code:
    ```rust
    use prkorm::Table;
    ```

2. Apply the #[derive(Table)] attribute to your struct. This will           automatically generate select(), insert(), update() and delete() methods for all the struct including but not limited to table_primary_key(), table(), select_str(), select_`field_name*`() Then you can chain functions join function, where, having, limit, order by, group by etc based on the type of query you are opting for.
Here is a quick example demonstrating the macro.
    ```rust
    #[derive(Table, Debug)]
    #[table_name("orders")]
    #[primary_key("id")]
    struct OrderModel {
        id: u32,
        customer_id: u32,
        address_id: u32,
        order_status: String,
        order_picture_url: String,
        created_at: String,
    }

    #[derive(Table, Debug)]
    #[table_name("customers")]
    struct Customer {
        id: u32,
        first_name: String,
        last_name: String,
    }
    ```

3. Use the generated methods as follows:

    ```rust
        let mut select_query: String = OrderModel::select()
        .select_str("CONCAT_WS(' ', `first_name`, `last_name`) as username ")
        .select_str("address_1")
        .left_join_by_customer_id(Customer::table(), "id")
        .left_join_by_address_id("addresses", "id")
        .order_by_created_at_desc()
        .where_order_status("PENDING")
        .having_order_status("PENDING")
        .build();

        println!("{}",select_query);
        //select_query:
        ```sql
        SELECT orders.id, orders.customer_id, 
        orders.address_id, orders.order_status, orders.order_picture_url, orders.created_at, 
        CONCAT_WS(' ', `first_name`, `last_name`) as username , address_1      
        FROM orders
        LEFT JOIN customers ON customers.id = orders.customer_id
        LEFT JOIN addresses ON addresses.id = orders.address_id
        WHERE orders.order_status = 'PENDING'
        HAVING orders.order_status = 'PENDING'
        ORDER BY orders.created_at DESC
        ```
    ```

## Examples
Here are a few examples of how to use the Table procedural macro:
```rust
use prkorm::Table;

 #[derive(Table, Debug)]
    #[table_name("orders")]
    struct OrderModel {
        id: u32,
        customer_id: u32,
        address_id: u32,
        order_status: String,
        order_picture_url: String,
        created_at: String,
    }

    #[derive(Table, Debug)]
    #[table_name("customers")]
    struct Customer {
        id: u32,
        mobile_number: u64,
        first_name: String,
        last_name: String,
    }

fn main() {
    // SELECT QUERY
    let mut select_query = OrderModel::select()
                            .where_customer_id_in(
                                Customer::select_id()
                                    .where_mobile_number_condition("!=","NULL")
                                    .build()
                                )
                            .build();

    // Output
    println!("{}", select_query);
    ```sql
    SELECT orders.id, orders.customer_id, orders.address_id, orders.order_status, orders.order_picture_url, orders.created_at
    FROM orders
    WHERE orders.customer_id IN (SELECT customers.id
    FROM customers
    WHERE customers.mobile_number != 'NULL')
    
    ```
    

    // INSERT QUERY
    let insert_query = Customer::insert()
                        .insert_to_first_name("Prakash")
                        // OR "9876543210"
                        .insert_to_mobile_number(9876543210u64) 
                        .build();

    // Output
    println!("{}", insert_query);

    ```sql
    INSERT INTO customers
    (first_name, mobile_number) VALUES  ('Prakash', '9876543210')
    ```

    
    // UPDATE QUERY: Note => No build() in update()
    let update_query =  Customer::update()
                        .update_first_name_with_value("JOHN")
                        .update_last_name_with_value("WICK")
                        .update_where_mobile_number_eq("9876543210");


    // Output
    println!("{}", update_query);

    ```sql
    UPDATE customers SET last_name = 'WICK', first_name = 'JOHN' 
    WHERE mobile_number = '9876543210'


    // DELETE QUERY: Note =>No build() in delete()
    let delete_query =  Customer::delete()
                        .delete_where_mobile_number_eq("9876543210");


    // Output
    println!("{}", update_query);

    ```sql
    DELETE FROM customers WHERE mobile_number = '9876543210'
    ```



}

```rust
#[derive(Table)]
#[table_name("posts")]
#[table_alias("P")]
#[primary_key("id")]
struct Post {
    id: i32,
    title: String,
    user_id: i32,
}

#[derive(Table)]
#[table_name("users")]
#[table_alias("U")]
struct User {
    id: i32,
    username: String,
}

fn main() {
    let query = User::select_str_as(
        &Post::select_function_over_field_name("COUNT", "*")
            .where_user_id("U.id")
            .build(),
        "total_post_count",
    )
    .where_id(1)
    .build();

    println!("{query}");
}

    ```sql
    SELECT 
    (SELECT COUNT(*) FROM posts P WHERE P.user_id = "U.id") AS total_post_count  
    FROM users U
    WHERE U.id = '1'
    ```

```
