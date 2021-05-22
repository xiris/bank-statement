table! {
    transactions (id) {
        id -> Nullable<Integer>,
        booking_date -> Text,
        value_date -> Text,
        booking_text -> Text,
        beneficiary -> Text,
        purpose -> Text,
        account_number -> Text,
        sort_code -> Text,
        amount -> Text,
        creditor_id -> Text,
        mandate_reference -> Text,
        customer_reference -> Text,
    }
}
