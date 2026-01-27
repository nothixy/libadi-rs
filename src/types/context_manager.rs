pub fn scan_contexts<'a>() -> Result<std::collections::HashMap<&'a str, &'a str>, ()> {
    let ctx = iio::IIOScanContext::new(None, 0);
    if let Some(mut context) = ctx {
        let ctx_nb = context.get_info_list()?;
        let mut map = std::collections::HashMap::new();
        for info in ctx_nb.as_slice().iter() {
            let uri = info.get_uri().map_err(|_| ())?;
            let description = info.get_description().map_err(|_| ())?;
            map.insert(uri, description);
        }
        Ok(map)
    } else {
        Err(())
    }
}
