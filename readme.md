
# temporary_mail

### Async Rust wrapper of [1secmail](https://www.1secmail.com/api) temporary mail service
Based on the [temporary_mail](https://github.com/DilecPadovani/temporary_mail)


```rust
use temporary_mail::TempMail;

let temp_mail = TempMail::new();
```
From `TempMail` you can retrieve:

## Email address 
```rust
println!("{}", temp_mail.get_address());
```

## Email inbox 
```rust
let emails: Result<Vec<Email>> = temp_mail.get_inbox().await;

// print received emails
if let Ok(emails) = emails {
    emails.iter().for_each(|mail| println!("{:?}", mail));
}
```





