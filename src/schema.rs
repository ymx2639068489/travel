// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        role_id -> Nullable<Varchar>,
        #[max_length = 36]
        company_id -> Nullable<Varchar>,
        #[max_length = 20]
        username -> Nullable<Varchar>,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 4000]
        avatar -> Nullable<Varchar>,
        #[max_length = 20]
        nickname -> Nullable<Varchar>,
    }
}

diesel::table! {
    base_product (id) {
        #[max_length = 36]
        id -> Varchar,
        create_at -> Timestamp,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        #[max_length = 500]
        file_list -> Nullable<Varchar>,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    company (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 20]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    custom (id) {
        id -> Integer,
        #[max_length = 10]
        name -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 20]
        id_type -> Nullable<Varchar>,
        #[max_length = 50]
        id_number -> Nullable<Varchar>,
        level -> Nullable<Integer>,
    }
}

diesel::table! {
    sales_records (id) {
        id -> Integer,
        custom_id -> Nullable<Integer>,
        salesman_id -> Nullable<Integer>,
        #[max_length = 36]
        product_id -> Nullable<Varchar>,
        create_at -> Timestamp,
        #[max_length = 20]
        company -> Varchar,
        #[max_length = 30]
        order_id -> Varchar,
        #[max_length = 30]
        pay_method -> Varchar,
        money -> Decimal,
        people_number -> Integer,
        #[max_length = 50]
        rebate -> Nullable<Varchar>,
    }
}

diesel::table! {
    ledger (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        product_id -> Nullable<Varchar>,
        #[max_length = 50]
        product_name -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        people_number -> Integer,
        #[max_length = 20]
        product_type -> Varchar,
        duration -> Integer,
        revenue -> Decimal,
        cost -> Decimal,
        #[max_length = 20]
        pay_status -> Varchar,
        #[max_length = 20]
        executor -> Varchar,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    operator (id) {
        id -> Integer,
        #[max_length = 36]
        admin_id -> Nullable<Varchar>,
        #[max_length = 30]
        teablename -> Varchar,
        #[max_length = 32]
        source_id -> Varchar,
        created_at -> Timestamp,
        #[max_length = 20]
        operator_type -> Varchar,
        #[max_length = 4000]
        origin_object -> Nullable<Varchar>,
        #[max_length = 4000]
        now_object -> Nullable<Varchar>,
        #[max_length = 300]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    product (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 36]
        base_product_id -> Nullable<Varchar>,
        create_at -> Timestamp,
        price -> Nullable<Decimal>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        people_number -> Integer,
        duration -> Integer,
        #[max_length = 20]
        product_type -> Varchar,
        #[max_length = 500]
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    role (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 20]
        rolename -> Varchar,
        #[max_length = 100]
        description -> Nullable<Varchar>,
        #[max_length = 4000]
        router -> Nullable<Varchar>,
        admin_value -> Integer,
        operator_value -> Integer,
        role_value -> Integer,
        company_value -> Integer,
        salesman_value -> Integer,
        sales_records_value -> Integer,
        product_value -> Integer,
        custom_value -> Integer,
    }
}

diesel::table! {
    salesman (id) {
        id -> Integer,
        #[max_length = 36]
        company_id -> Nullable<Varchar>,
        #[max_length = 20]
        username -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
    }
}

diesel::joinable!(admin -> company (company_id));
diesel::joinable!(admin -> role (role_id));
diesel::joinable!(sales_records -> custom (custom_id));
diesel::joinable!(sales_records -> product (product_id));
diesel::joinable!(sales_records -> salesman (salesman_id));
diesel::joinable!(operator -> admin (admin_id));
diesel::joinable!(product -> base_product (base_product_id));
diesel::joinable!(salesman -> company (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    base_product,
    company,
    custom,
    sales_records,
    ledger,
    operator,
    product,
    role,
    salesman,
);
