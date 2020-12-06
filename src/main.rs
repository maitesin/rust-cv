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
                .text("\nI am a {mod=bold DevOps Engineer} interested in {mod=bold Architecting} and {mod=bold Infrastructure Automation}.\n\n\
                       I am a regular attendee of the {mod=bold AWS} and {mod=bold Go} MeetUps Online.\n\n\
                       I ride {mod=bold Cool Bikes} at {mod=bold Flying level} and making my own {mod=bold Sounds}.\n\n\
                       I enjoy programing a complete {mod=bold Live Show Production} for {mod=bold Samarcanda Entertainment} in Croatia.\n\n\
                       A big fan of {mod=bold Star Wars} and {mod=bold Human} lover.\n\n\
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
                    .label(&format!("60 / 100"))
                    .percent(60)
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
                    .block(Block::default().title("GNU/Linux").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[0]);
                Gauge::default()
                    .block(Block::default().title("MacOS").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                    .style(Style::default()
                            .fg(Color::Magenta)
                            .bg(Color::Black)
                            .modifier(Modifier::Italic))
                    .label(&format!("95 / 100"))
                    .percent(95)
                    .render(t, &chunks[1]);
                Gauge::default()
                    .block(Block::default().title("Windows").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
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
                        .items(&vec!["Architectural", "Thinking", "", "Analicital", "", "Responsiveness", "", "Cominication", "", "Problem Solving"])
                        .render(t, &chunks[1]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Languages").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Python", "", "Rust", "", "Lua"])
                        .render(t, &chunks[3]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("CI/CD").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Gitlab CI", "", "Git Runner"])
                        .render(t, &chunks[5]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Monitoring").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["CloudWatch"])
                        .render(t, &chunks[7]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Databases").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["RDS", "", "MySQL", "", "Aurora"])
                        .render(t, &chunks[9]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("IDE").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["InteliJ", "", "Visual Studio", "", "Doker Hub"])
                        .render(t, &chunks[11]);
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title("Honors").title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
                        .items(&vec!["Best Employee", "2019", "2015", "", "Bike Park", "Demonstration", "", "1st Place in", "Swimming Competition", "", "Design Award"])
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
                .text("\n{mod=bold;fg=yellow Organisation:} Spiking technologies and resolve impediments as Scrum Master.\n\ Designing and Building Infrastructures as Code by implementing Best 'Practices' and automated Build/Test/Deploy (CICD).\n\n\
                       {mod=bold;fg=yellow Terraform:} Infrastructure as Code Refactoring from CloudFormation to Terraform to reduce the number of CF-Stack and better deployment.\n\n\
                       {mod=bold;fg=yellow Others:} Automating CI/CD for Microservices Deployable as Docker Container to Kubernetes(EKS).\n\n\
                       ELB, ASG, EC2, S3, Bastion, KMS, IAM, EBeanstalk, CloudFormation, Terraform, Terragrunt, Ansible, Docker, Kubernetes(EKS), VPC peering, RDS MSQL, Aurora PHP, AWS CLI, Ubuntu, RedHat8, Automation Scripting, Springboot, GitRunner, SSM Parameter Store, Nested Stacks, GitLab, Jira, Confluence, Agile, Scrum Master, Kanban, Documentation, Architecture Design\n\n\
                      ")
                .render(t, &chunks[0]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2011 - 2020: Tech Executive Produceer at Samarcanda Entertainment, Life Tourism S.A, Self Employed")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow General:} Coaching and of new potential employees.\n\n\
                 Providing a personal project in quality and innovations for sound and light equipment and the concept of operation together with external associates. Executing technical operation of live show productions and special events in the area of a international leading tourist destination.\n\Show programming, recording, live sound, video recording, video editing, 3D mapping, multiscreen projection, PR, consulting.\n\n\
                       {mod=bold;fg=yellow Nice:} Guinness World Records (Zaton Smiley 2015) Nominated for best employee 2015\n\Technical Coach at Sama Academy 2019 Nominated for best employee 2019.\n\n\
                       {mod=bold;fg=yellow Location:} Multiple Locations in Europa.\n\n\
                      ")
                .render(t, &chunks[1]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2018 - 2019: Warehouse Logistics at CNH Industrial GmbH (3rd party employment)")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow General:} Executing orders in a Automotive field of special parts and components for industrial machines..\n\n\
                       {mod=bold;fg=yellow Quality Management:} Problem Solving alongside with Warehouse Operation System (WOS), Inventory Check and Participating in Quality Management Projects.\n\n\
                       {mod=bold;fg=yellow Achivement:} Found system mistakes of big values they probably would never find.\n\n\
                      ")
                .render(t, &chunks[2]);
                Paragraph::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("2010 - 2010: Corn Care at BC Institute ")
                    .title_style(Style::default().fg(Color::Green).modifier(Modifier::Bold)))
                .wrap(true)
                .text("\n{mod=bold;fg=yellow General:} Dying on +40 degree in Sun, snakes and no water.\n\n\
                       {mod=bold;fg=yellow Decision:} This is not my future...\n\n\

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
            .title("AWS Solutions Architect - Associate (from Linux Academy) - March 2020")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt how to setup, mantain and use a various kind of infrastructures, including how to deploy a containerized application and manipulating resources.")
        .render(t, &chunks[0]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("SSM Certified SAFe® 4 Scrum Master (from Scaled Agile Framework) - Dezember 2019")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt about the differenties in Frameworks and the strenght of Agile methodology, especialy using Kanban and their User Stories in Jira.")
        .render(t, &chunks[1]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Linux Networking and Subneting (from Linux Academy) - Dezember 2019")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt what OSI Layers and CIDR blocks are, the basic of Subnet masks, Hosts and IP ranges.")
        .render(t, &chunks[2]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Introduction to Computer Science (from EDX) - August 2015")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nLearnt how to think algorithmically and solve problems efficiently.")
        .render(t, &chunks[3]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Mastering Desing Thinking (from MIT) - in progress")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\n...501...")
        .render(t, &chunks[4]);
        Paragraph::default()
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Live Sound Engineering (from LinkedIn) - September 2018")
            .title_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold)))
        .wrap(true)
        .text("\nConfirming skills in Live Sound for Live Show Productions.")
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
