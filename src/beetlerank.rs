#[cfg(test)]
mod tests {
    use anyhow::Result;
    use serde::Deserialize;


    #[derive(Deserialize)]
    struct DevResponse {
        succeed: bool
    }

    #[derive(Deserialize)]
    struct Cups {
        cups: Vec<String>
    }

    #[derive(Deserialize)]
    struct Maps {
        maps: Vec<String>
    }

    #[test]
    fn test_motd() -> Result<()> {
        let url = "http://localhost:3000/api/dev/info";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_devapi() -> Result<()> {
        let url = "http://localhost:3000/api/dev/info";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_cups() -> Result<()> {
        let url = "http://localhost:3000/api/dev/cups";
        let response: Cups = reqwest::blocking::get(url)?.json()?;
        assert_eq!(response.cups, vec![
            String::from("TYRIA CUP"),
            String::from("GUILDHALL CUP")
        ]);
        Ok(())
    }

    #[test]
    fn test_maps() -> Result<()> {
        let url = "http://localhost:3000/api/dev/maps/TYRIA CUP";
        let response: Maps = reqwest::blocking::get(url)?.json()?;
        assert_eq!(response.maps, vec![
            String::from("TYRIA GENDARRAN")
        ]);
        Ok(())
    }

    #[test]
    fn test_map_rankings() -> Result<()> {
        let url = "http://localhost:3000/api/dev/top3/DEV";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }

    #[test]
    fn test_user_rankings() -> Result<()> {
        let url = "http://localhost:3000/api/dev/top3/DEV/Test User";
        let response: DevResponse = reqwest::blocking::get(url)?.json()?;
        assert!(response.succeed);
        Ok(())
    }
}