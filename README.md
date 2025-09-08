# Superhero_Tournament
A turn-based fighting game like Pokemon Stadium but with in fight cutscenes similar to Dragon Ball Gokuden Legend Of The Super Sayian but it is text-based and a roguelike

<h2>The instructions are used for Linux Mint so look for differences with your OS and adapt these instructions to your OS</h2>

<h3>Prerequisites for Rust and Application</h3>
<ol>
  <li>Install rust from here: https://www.rust-lang.org/tools/install</li>
  <li>Install Lua 5.4 development files on your machine, type this in terminal to install: sudo apt update
sudo apt install liblua5.4-dev pkg-config
then press enter</li>
</ol>

<h3>How to compile Superhero_Tournament, Warning: the repository folder is the folder i am talking about or a folder with the repositories contents in it.</h3>
<ol>
  <li>Put this in the terminal first then press enter except the stuff that is in parentheses and parentheses: cd directory(directory must be unique to those 2 files and that will be your project folder) with Superhero_Tournament.rs and Cargo.toml file</li>
  <li>Put this in the terminal then press enter except the stuff that is in parentheses and parentheses: cargo build (for fast compilation but not efficent application) or cargo build --release (for efficent application but not fast compilation)</li>
  <li>The executable file will be in your project folder in /target/debug or release directory</li>
  <li>To open the executable you just drag the executable into your terminal and press enter.</li>
</ol>

<h3>Mod Support Info</h3>
<ul>
  <li>To load a mod file, the mod file needs to be in cloned repository folder/folder with Superhero_Tournament.rs and Cargo.toml files, out of its folder and it needs to be and needs to be called Superhero_Tournament_Mod.lua</li>
</ul>

<a href="https://github.com/Daniel-Hanrahan-Tools-and-Games/Superhero_Tournament_Mod">Example Mod Repository Page</a>

<a href="https://github.com/Daniel-Hanrahan-Tools-and-Games/Superhero_Tournament">Repository Page</a>

<a href="https://daniel-hanrahan-tools-and-games.github.io/">Home Page</a>
