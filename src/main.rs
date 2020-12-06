extern crate tui;
extern crate termion;

use std::io;
use std::thread;
use std::time;
use std::sync::mpsc;

use termion::event;
use termion::input::TermRead;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, SelectableList, Gauge, Paragraph, Borders, Tabs};
use tui::layout::{Group, Direction, Size, Rect};
use tui::style::{Style, Color, Modifier};

pub struct MyTabs<'a> {
    pub titles: Vec<&'a str>,
    pub selection: usize,
}

impl<'a> MyTabs<'a> {
    pub fn next(&mut self) {
        self.selection = (self.selection + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.selection > 0 {
            self.selection -= 1;
        } else {
            self.selection = self.titles.len() - 1;
        }
    }
}

struct App<'a> {
    size: Rect,
    tabs: MyTabs<'a>
}

enum Event {
    Input(event::Key),
    Tick,
}

fn main() {
    let mut app = App {
        size: Rect::default(),
        tabs: MyTabs {
            titles: vec!["Welcome", "Personal", "Skills", "Experience", "Courses", "Looking For"],
            selection: 0,
        }
    };
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            input_tx.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    thread::spawn(move || {
        let tx = tx.clone();
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    loop {
        let size = terminal.size().unwrap();
        if size != app.size {
            terminal.resize(size).unwrap();
            app.size = size;
        }
        draw(&mut terminal, &app).unwrap();
        let evt = rx.recv().unwrap();
        match evt {
            Event::Input(input) => {
                match input {
                    event::Key::Char('q') => {
                        break;
                    }
                    event::Key::Left => {
                        app.tabs.previous();
                    }
                    event::Key::Right => {
                        app.tabs.next();
                    }
                    _ => {}
                }
            }
            Event::Tick => {}
        }
    }
    terminal.show_cursor().unwrap();
    terminal.clear().unwrap();
}

fn draw(t: &mut Terminal<RawBackend>, app: &App) -> Result<(), io::Error> {

    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Fixed(3), Size::Min(0)])
        .render(t, &app.size, |t, chunks| {
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .style(Style::default().fg(Color::Green))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(app.tabs.selection)
                .render(t, &chunks[0]);
            match app.tabs.selection {
                0 => {
                    draw_welcome(t, &chunks[1]);
                }
                1 => {
                    draw_personal(t, &chunks[1]);
                }
                2 => {
                    draw_skills(t, &chunks[1]);
                }
                3 => {
                    draw_experience(t, &chunks[1]);
                }
                4 => {
                    draw_education(t, &chunks[1]);
                }
                5 => {
                    draw_looking_for(t, &chunks[1]);
                }
                _ => {}
            };
        });
    try!(t.draw());
    Ok(())
}

fn draw_welcome(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("Welcome to Marko Dujmovic Curriculum Vitae")
                        .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\nUse {mod=bold;fg=yellow ←}  and {mod=bold;fg=yellow →}  to move between the tabs.\n\n\
                           Use {mod=bold;fg=yellow q} to exit the application.\n\n\
                           I hope you like it!\n\n\
                           {mod=bold;fg=yellow **Note:} Optimized resolution of the command line is 120x40 characters.{mod=bold;fg=yellow **}\
                           ")
                    .render(t, &chunks[1]);
                });
        });
}

fn draw_personal(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(50), Size::Percent(50)])
        .render(t, area, |t, chunks| {
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(35), Size::Percent(65)])
            .render(t, &chunks[0], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Information")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Name:} Marko Dujmovic\n\n\
                       {mod=bold;fg=yellow Date of Birth:} 25/03/1990\n\n\
                       {mod=bold;fg=yellow Nationality:} Croatian\n\n\
                       {mod=bold;fg=yellow Location:} Zagreb CRO\n\n\
                       {mod=bold;fg=yellow Open to relocation within the E.U.}\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("About me")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nI am a AWS Soulutions Architect interested in {mod=bold Infrastructure Architecting} and {mod=bold Systems Automation}.\n\n\
                       I am a regular attendee of the {mod=bold AWS} and {mod=bold Go} MeetUps Online.\n\n\
                       I ride {mod=bold Cool Bikes} at {mod=bold Flying level} and making my own {mod=bold Sounds}.\n\n\
                       I enjoy programing a complete {mod=bold Live Show Production} for {mod=bold Samarcanda Entertainment} in Croatia.\n\n\
                       A big fan of {mod=bold Star Wars} and {mod=bold Meeting Humans} lover.\n\n\
                      ")
                .render(t, &chunks[1]);
            });
            Group::default()
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(20), Size::Percent(45), Size::Percent(35)])
            .render(t, &chunks[1], |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Languages")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Croatian:} Native\n\n\
                       {mod=bold;fg=yellow German:} Native\n\n\
                       {mod=bold;fg=yellow English:} Fluent\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Education")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Harvard University - Computer Science (50)}\n\
                        abstraction, algorithms, data structures,\n\
                        encapsulation, resource management, security,\n\
                        software engineering, and web development...\n\
                        Familiarity in a number of languages, including\n\
                        C, Python, SQL, and JavaScript plus CSS and HTML.\n\n\

                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Contact")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Email:} marko.dujmovic@yahoo.com\n\n\
                       {mod=bold;fg=yellow Phone:} +385 (0) 919542289\n\n\
                       {mod=bold;fg=yellow Website:} SOON!\n\n\
                       {mod=bold;fg=yellow LinkedIn:} https://bit.ly/37Imv5x\n\n\
                      ")
                .render(t, &chunks[2]);
            });
        });
}

fn draw_skills(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(35), Size::Percent(35), Size::Percent(30)])
        .render(t, area, |t, chunks| {
            Block::default()
                .borders(Borders::ALL)
                .title("Tech Stack")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[0], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("AWS").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("85 / 100"))
                    .percent(85)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("Terraform").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("Bash").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[2]);
                Gauge::default()
                    .block(Block::default().title("Docker").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("75 / 100"))
                    .percent(75)
                    .render(t, &chunks[3]);
                Gauge::default()
                    .block(Block::default().title("Kubernetes(EKS)").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("75 / 100"))
                    .percent(75)
                    .render(t, &chunks[3]);
            });
            Block::default()
                .borders(Borders::ALL)
                .title("Operating Systems")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[1]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[1], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("GNU/Linux:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("MacOS:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("Windows:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("60 / 100"))
                    .percent(60)
                    .render(t, &chunks[2]);
            });
            Block::default()
                .borders(Borders::ALL)
                .title("Other Skills")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[2]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Fixed(2), Size::Fixed(18),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(13),Size::Fixed(2),Size::Fixed(19),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(16),Size::Fixed(2),Size::Fixed(17),Size::Fixed(2)])
                .render(t, &chunks[2], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Mind").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Architectural Thinking", "Analitical Thinking", "Responsiveness", "Cominication", "Problem Solving"])
                        .render(t, &chunks[1]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Languages").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Python", "Rust", "Lua"])
                        .render(t, &chunks[3]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("CI/CD").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Gitlab CI", "Git Runner"])
                        .render(t, &chunks[5]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Monitoring").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["CloudWatch"])
                        .render(t, &chunks[7]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Databases").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["RDS", "MySQL", "Aurora"])
                        .render(t, &chunks[9]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("IDE").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["InteliJ", "Visual Studio" , "Doker Hub"])
                        .render(t, &chunks[11]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Honors").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["AWS Feedback Ninja", "Best Employee 2019", "Best Employee 2015", "Bike Park Demonstration", "1st Place in Swimming Competition", "Design Award"])
                        .render(t, &chunks[13]);
            });
    });
}

fn draw_experience(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(30), Size::Percent(30),Size::Percent(20), Size::Percent(20)])
        .render(t, area, |t, chunks| {
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2019 - 2020: DevOps Engineer at Appon GmbH / d.o.o.")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow High Performance Platform:} Implemented new functionalities that serve thousands of certificates per second. Took care of the migration of backend storage from MongoDB to PostgreSQL.\n\n\
                       {mod=bold;fg=yellow Kubernetes prototype:} Implemented a prototype of the system used in production to test the feasibility of a future migration.\n\n\
                       {mod=bold;fg=yellow Others:} Mentored multiple new-hires and had a highly involved role in the hiring process. As a side project I developed an static analysis tool to enforce the code style used in the company.\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2016 - 2018: Software Engineer at VCA Technology")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Network library:} Allows the user to interact with Boost ASIO through the std::stream interface.\n\n\
                       {mod=bold;fg=yellow Tools:} Improve the toolchain used internally to allow the usage of clang-tidy for the linting of our code. Another tool allows to switch between several versions of the toolchain without having to change the environment.\n\n\
                       {mod=bold;fg=yellow Backend development:} Several activities regarding the extension and implementation of new features in the backend of the product such as Authentication and GStreamer sinks.\n\n\
                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2015 - 2016: Software Developer at Programming Research")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Static analysis of code:} Checks if the code has some patterns that have an undefined behaviour, unspecified in the Standard, and/or implementation-defined.\n\n\
                       {mod=bold;fg=yellow Dataflow analysis of code:} Checks the complexity of methods, pointer problems, memory handling, etc.\n\n\
                       Took over two projects to refactor, maintain and add new features.\n\n\
                      ")
                .render(t, &chunks[2]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2013 - 2015: Software Engineer at European Bioinformatics Institute")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow RESTful service:} Allows  users to query for information about complexes in the database.\n\n\
                       {mod=bold;fg=yellow Cluster algorithm:} Developed a new algorithm to cluster biological information from proteins.\n\n\
                      ")
                .render(t, &chunks[3]);
    });
}

fn draw_education(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(18), Size::Percent(18),Size::Percent(19), Size::Percent(15),Size::Percent(15),Size::Percent(15)])
        .render(t, area, |t, chunks| {
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("LFS258: Kubernetes Fundamentals (from Linux Foundation) - August 2019")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt how to setup, mantain and use a Kubernetes cluster, including how to deploy a containerized application and manipulating resources via the API.")
        .render(t, &chunks[0]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("LFD331: Developing Linux Device Drivers (from Linux Foundation) - April 2016")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt about the different Linux device drivers, APIs and methods through which devices interface with the kernel.")
        .render(t, &chunks[1]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("LFD320: Linux Kernel Internals and Debugging (from Linux Foundation) - March 2016")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt how Linux is architected, the basic methods for developing on the kernel, and how to work with the community.")
        .render(t, &chunks[2]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Agile for Developers (from Accelebrate) - August 2015")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nTeaches intermediate and advanced object-oriented developers the practices of Agile and Scrum.")
        .render(t, &chunks[3]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Algorithms, Part II (from Coursera) - November 2014")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nFocuses on graph, and string processing algorithms.")
        .render(t, &chunks[4]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Algorithms, Part I (from Coursera) - September 2014")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nCovers elementary data structures, sorting, and searching algorithms.")
        .render(t, &chunks[5]);
    });
}

fn draw_looking_for(t: &mut Terminal<RawBackend>, area: &Rect) {
    Group::default()
        .direction(Direction::Vertical)
        .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
        .render(t, area, |t, chunks| {
                Group::default()
                .direction(Direction::Horizontal)
                .sizes(&[Size::Percent(10), Size::Percent(80), Size::Percent(10)])
                .render(t, &chunks[1], |t, chunks| {
                    Paragraph::default()
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .title("What I am looking for?")
                        .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                    .wrap(true)
                    .text("\n{mod=bold;fg=yellow I am currently looking for new opportunities}\n\n\n\
                           My ideal roles involve a combination of the following:\n\n\
                           \t* Write low level libraries and/or components.\n\
                           \t* Design, develop and maintain a high performance and reliable systems.\n\
                           \t* Create and integrate APIs to expose and extend the functionality.\n\
                           \t* Create and improve the tools used during the development process.\n\
                           \t* Work in the internals of Operating Systems such as GNU/Linux and FreeBSD.\n\
                           \t* Work on compilers and/or interpreters and designing programming languages.\n\
                           \t* Contribute to Open Source software.\
                           ")
                    .render(t, &chunks[1]);
                });
        });
}
