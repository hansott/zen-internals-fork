macro_rules! is_injection {
    ($query:expr, $input:expr) => {
        assert!(detect_sql_injection_str($query, $input, 0))
    };
}
macro_rules! not_is_injection {
    ($query:expr, $input:expr) => {
        assert!(!detect_sql_injection_str($query, $input, 0))
    };
}
#[cfg(test)]
mod tests {
    use crate::sql_injection::detect_sql_injection::detect_sql_injection_str;

    #[test]
    fn test_injection_with_token_counts() {
        is_injection!(
            "INSERT INTO users (name, age, email, city) VALUES ('Alice');;;;;;;    -- ', 30, 'alice@example.com', 'Wonderland'); -- Hellow",
            "Alice');;;;;;;    --"
        );
        is_injection!(
            "INSERT INTO users (name, age, email, city) VALUES ('Alice');;;;;;;  -- ', 30, 'alice@example.com', 'Wonderland');",
            "Alice');;;;;;;  --"
        );
    }

    #[test]
    fn test_encapsulated_strings() {
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "Alice"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "Bob"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('W.Al294*is', 'Bob')",
            "W.Al294*is"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('John', 'Doe')",
            "'John'"
        );

        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('John', 'Doe')",
            "John', 'Doe"
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('John', 'Doe')",
            "John',"
        );
    }

    #[test]
    fn test_keywords() {
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "INSERT"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "INS"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "users"
        );
        not_is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "VALUES"
        );

        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "INSERT INTO"
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "INSERT INTO users"
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "INTO users"
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "VALUES "
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            " INTO"
        );
    }
    #[test]
    fn test_character_combos() {
        // SQL Injection :
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "('"
        );
        is_injection!(
            "INSERT INTO users (name, surname) VALUES ('Alice', 'Bob')",
            ", "
        );
        is_injection!(
            "INSERT  INTO users (name, surname) VALUES ('Alice', 'Bob')",
            "  "
        );
    }
}
