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
                        .title("Welcome to Oscar Forner's Curriculum Vitae")
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
                .text("\n{mod=bold;fg=yellow Name:} Oscar Forner Martinez\n\n\
                       {mod=bold;fg=yellow Date of Birth:} 24/03/1988\n\n\
                       {mod=bold;fg=yellow Nationality:} Spanish\n\n\
                       {mod=bold;fg=yellow Location:} London UK\n\n\
                       {mod=bold;fg=yellow Open to relocation within the E.U.}\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("About me")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\nI am a Software Engineer interested in {mod=bold Backend development} and {mod=bold Systems Programming}.\n\n\
                       I am a regular attendee of the {mod=bold C++} and {mod=bold Go} MeetUps in London.\n\n\
                       I play {mod=bold handball} at {mod=bold national level} for the {mod=bold Chelsea Team} in London.\n\n\
                       I enjoy playing all sorts of {mod=bold board games}, like {mod=bold Magic the Gathering}, and {mod=bold videogames}.\n\n\
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
                .text("\n{mod=bold;fg=yellow Spanish:} Native\n\n\
                       {mod=bold;fg=yellow Catalan:} Native\n\n\
                       {mod=bold;fg=yellow English:} Fluent\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Studies")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow 2010 - 2013: Bachelor of Engineering in Computer Science}\n\
                        Advanced Data Structures, Compilers and Interpreters,\n\
                        Advanced Operating Systems, and Distributed Systems.\n\n\
                       {mod=bold;fg=yellow 2006 - 2010: Associate Degree in Computer Science}\n\
                        Operating Systems, Real Time Operating Systems and\n\
                        Embedded Systems.\n\n\
                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Contact")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow Email:} oscar.forner.martinez@gmail.com\n\n\
                       {mod=bold;fg=yellow Phone:} +44 (0) 7596944383\n\n\
                       {mod=bold;fg=yellow Website:} https://oscarforner.com/\n\n\
                       {mod=bold;fg=yellow Twitter:} https://twitter.com/oscar_forner\n\n\
                       {mod=bold;fg=yellow GitHub:} https://github.com/maitesin\n\n\
                       {mod=bold;fg=yellow LinkedIn:} http://linkedin.com/in/oscarforner\n\n\
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
                .title("Programming Languages")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[0]);
            Group::default()
                .direction(Direction::Vertical)
                .margin(1)
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
                .render(t, &chunks[0], |t, chunks| {
                Gauge::default()
                    .block(Block::default().title("C++ (11/14/17):").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("90 / 100"))
                    .percent(90)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("Go:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("85 / 100"))
                    .percent(85)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("Python:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[2]);
                Gauge::default()
                    .block(Block::default().title("C:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
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
                .sizes(&[Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2),Size::Fixed(2)])
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
                    .block(Block::default().title("FreeBSD:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("80 / 100"))
                    .percent(80)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("OpenBSD:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("70 / 100"))
                    .percent(70)
                    .render(t, &chunks[2]);
                Gauge::default()
                    .block(Block::default().title("MacOS:").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("65 / 100"))
                    .percent(65)
                    .render(t, &chunks[3]);
            });
            Block::default()
                .borders(Borders::ALL)
                .title("Others")
                .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold))
                .render(t, &chunks[2]);
            Group::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .sizes(&[Size::Fixed(2), Size::Fixed(18),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(13),Size::Fixed(2),Size::Fixed(19),Size::Fixed(2),Size::Fixed(15),Size::Fixed(2),Size::Fixed(16),Size::Fixed(2),Size::Fixed(17),Size::Fixed(2)])
                .render(t, &chunks[2], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Orchestration").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Kubernetes", "Docker Compose", "Helm"])
                        .render(t, &chunks[1]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Build Systems").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["CMake", "Meson", "Make", "Ninja"])
                        .render(t, &chunks[3]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("CI/CD").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Gitlab CI", "Jenkins", "Bamboo", "Travis"])
                        .render(t, &chunks[5]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Analysis").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Golint", "Go vet", "Clang-sanitizer", "Perf", "Valgrind"])
                        .render(t, &chunks[7]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Databases").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["MongoDB", "PostgreSQL", "SQLite", "CockroachDB", "MariaDB", "MySQL"])
                        .render(t, &chunks[9]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Message Broker").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["RabbitMQ", "Kafka"])
                        .render(t, &chunks[11]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Hypervisors").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["KVM", "VirtualBox", "VMWare", "OpenBSD's VMM"])
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
                    .title("2018 - Present: Senior Software Engineer at GlobalSign")
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
