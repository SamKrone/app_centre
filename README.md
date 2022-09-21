# app_centre
A CLI program written in Rust that allows you to quickly and discreetly access applications from a CLI interface

Apps and app locations are stored in folders. This is stored and represented in the apps.json file as such:

```
{
    "Example Category 1":
    {
        "Example App 1": "C:/ExampleApp1.exe",
        "Example App 2": "C:/ExampleApp2.exe",
        "Example App 3": "C:/ExampleApp3.exe"
    },
    "Example Category 2":
    {
        "Example App 4": "C:/ExampleApp4",
        "Example App 5": "C:/ExampleApp5",
        "Example App 6": "C:/ExampleApp6"
    }
}
```

Where...

```
"Example Category" is the category name

"Example App" is the displayed name of the app

"C:/ExampleApp" is the file location of the app
```


apps.json needs to be edited to add new apps or folders to the program
