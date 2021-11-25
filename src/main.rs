use std::{io, process::exit};
use tui::{
	backend::CrosstermBackend,
	style::{Color, Style},
	widgets::{Block, Borders, List, ListItem},
	Terminal,
};

fn main_menu<B: tui::backend::Backend>(mut terminal: Terminal<B>) -> anyhow::Result<()> {
	terminal.clear()?;
	let items = [ListItem::new("query vocabulary"), ListItem::new("add vocabulary")];
	let list = List::new(items)
		.block(Block::default().title("Main Menu").borders(Borders::ALL))
		.style(Style::default().fg(Color::White));
	terminal.draw(|f| {
		let size = f.size();
		f.render_widget(list, size);
	})?;
	Ok(())
}

fn main() -> Result<(), io::Error> {
	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	if let Err(error) = main_menu(terminal) {
		eprintln!("{:?}", error);
		exit(1);
	}
	Ok(())
}
