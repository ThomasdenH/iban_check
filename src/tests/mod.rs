use validate_iban;

#[test]
/// This test checks whether IBANs of invalid lengths are detected to be invalid.
fn test_length() {
    let invalid_lengths = ["DE4", "DE445001023460732493147896512575467"];

    validate_all(&invalid_lengths, false);
}

#[test]
/// This test checks whether IBANs containing invalid characters are detected to be invalid.
fn test_characters() {
    let invalid_characters = ["DE44@0010234607324931",
                              "DE44@0010234607324931",
                              "$A0380000000648510167519",
                              "tr330006100519786457465326",
                              "G416011012500000834112300695",
                              "CHI300762011623852957"];

    validate_all(&invalid_characters, false);
}

#[test]
/// This test checks whether IBANs having an invalid checksum are detected as such.
fn test_checksum() {

    let invalid_checksums = ["DE4450010234607324931",
                             "GR16011012500000834112300695",
                             "GB29NWBK60934331926819",
                             "SA0380000000648510167519",
                             "CH9300762011645852957",
                             "TR330006100519786457465326"];

    validate_all(&invalid_checksums, false);
}

#[test]
/// This test checks whether valid IBANs are marked valid.
fn test_valid_iban() {

    let valid_ibans = ["DE44500105175407324931",
                       "GR1601101250000000012300695",
                       "GB29NWBK60161331926819",
                       "SA0380000000608010167519",
                       "CH9300762011623852957",
                       "TR330006100519786457841326"];

    validate_all(&valid_ibans, true);
}

/// This is a helper method that asserts whether all IBANs in a list give the given result.
fn validate_all(numbers: &[&str], result: bool) {
    for &n in numbers {
        assert_eq!(validate_iban(n), result);
    }
}
