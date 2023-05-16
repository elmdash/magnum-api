use magnum_api::magnum_type;

macro_rules! assert_rep {
    ($item:expr, $exp:expr) => {
        assert_eq!(serde_json::to_string(&$item).unwrap(), $exp);
    };
}

#[test]
pub fn derives_serde() {
    #[magnum_type]
    struct Toaster {
        slots: usize,
    }

    assert_rep!(Toaster { slots: 2 }, r#"{"slots":2}"#);
}

#[test]
pub fn derives_serde_nested() {
    #[magnum_type]
    struct Toaster {
        slots: usize,
    }
    #[magnum_type]
    struct Kitchen {
        toaster: Toaster,
    }

    assert_rep!(
        Kitchen {
            toaster: Toaster { slots: 2 }
        },
        r#"{"toaster":{"slots":2}}"#
    );
}
