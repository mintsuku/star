# Star ⭐ - Tar-Based Command Line Utility 

Star is a command-line utility written in Rust that provides easy-to-use functions for working with tar files. Whether you need to extract files from tar archives or search for specific files, Star has got you covered. This tool is heavily focused on the ergonomics and usability of this tool to create a smooth and intuitive user experience.

## Features  

- Extract files from tar archives (supports both gzipped and xzipped tar files)
- Search for files within a directory by keyword  

## Prerequisites  

Before you begin, ensure you have met the following requirements:  

- [Rust](https://www.rust-lang.org/): You should have Rust installed on your system. If not, you can install it here.   

## Installation  

To install Star, follow these steps:  

```bash  
git clone https://github.com/shelovesmox/star.git  
```  

```bash   
cd star  
cargo build --release  
```  

```bash  
$HOME/star/target/release
``` 

- #### I have not published to crates.io yet, I will soon.


## Usage  

### Extracting Files  

To extract files from a .gz tar archive, use the following command:  

```bash  
star -gz <FILE>  
```  

Replace `<FILE>` with the path to the tar archive you want to extract. This command is for extracting files from gzipped.


To extract files from a .xz tar archive, use the following command:  

```bash  
star -xz <FILE>  
```  
Replace `<FILE>` with the path to the tar archive you want to extract. This command is for extracting files from xzipped.

### Searching for Files   

To search for files within a directory by keyword, use the following command:  

```bash   
star -search <KEYWORD>    
``` 

### Getting help


To get help, use the following command:
```
star -h
```


## TODO

- [ ] Add support for bzip2 compressed tar files
- [ ] Implement wildcard search
- [ ] Create man pages
- [ ] Implement search by date/time
- [x] Add verbosity levels
- [ ] Add unit and integration tests
- [ ] Support tar file creation
- [ ] Implement incremental backup feature
- [ ] Support different compression levels
- [ ] Handle large files more efficiently


## Contributing  

Contributions are welcome! Please feel free to submit a pull request or open an issue if you find any bugs or have suggestions for improvements.  

## License  

This project is licensed under the MIT License - see the LICENSE file for details.   

## Acknowledgments   

Thanks to the Rust community for providing a fantastic programming language.  

Special thanks to contributors who help improve this tool.  

Happy tar-ing with Star! 🌟