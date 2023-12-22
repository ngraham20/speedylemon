## Design Principals
- Comprehensive application
- Track speed
- Understand checkpoints
- Interface with LiveSplit for timer
- Integrate timer at some point

- Modular and well-documented
- Test-driven development

- The checkpoint creator might be a separate program, or may be integrated# Beetlerank API

## GET
- MOTD: `https://www.beetlerank.com/api/info`
- Cups: `https://www.beetlerank.com/api/cups`
- Maps: `https://www.beetlerank.com/api/maps/<name CUP>`
- Rank: `https://www.beetlerank.com/rank/api/<course>/<username>`

## POST
`https://www.beetlerank.com/upload-log`
- `user: <username>`
- `guildhall: <course>`
- `file: <logfile>`

# Debugging
Logging is handled with the environment variable `RUST_LOG`. Set it to **error**, **info**, **debug**, or **trace** for each desired log level.

Logging for release builds is disabled, so the while during debugging, you may see log messages clutter your console application, these will 
not be present in the release build. You can see this for yourself by running `cargo run --release`
