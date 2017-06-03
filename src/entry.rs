use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::events::attributes::Attributes;
use quick_xml::reader::Reader;

use category::Category;
use content::Content;
use error::Error;
use fromxml::FromXml;
use link::Link;
use person::Person;
use source::Source;
use util::atom_text;

/// Represents an entry in an Atom feed
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Entry {
    /// A human-readable title for the entry.
    title: String,
    /// A universally unique and permanent URI.
    id: String,
    /// The last time the entry was modified.
    updated: String,
    /// The authors of the feed.
    authors: Vec<Person>,
    /// The categories that the entry belongs to.
    categories: Vec<Category>,
    /// The contributors to the entry.
    contributors: Vec<Person>,
    /// The Web pages related to the entry.
    links: Vec<Link>,
    /// The time of the initial creation or first availability of the entry.
    published: Option<String>,
    /// The source information if an entry is copied from one feed into another feed.
    source: Option<Source>,
    /// A short summary, abstract, or excerpt of the entry.
    summary: Option<String>,
    /// Information about rights held in and over the entry.
    rights: Option<String>,
    /// Contains or links to the complete content of the entry.
    content: Option<Content>,
}

impl Entry {
    /// Return the title of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_title("Entry Title");
    /// assert_eq!(entry.title(), "Entry Title");
    /// ```
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Set the title of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_title("Entry Title");
    /// ```
    pub fn set_title<V>(&mut self, title: V)
        where V: Into<String>
    {
        self.title = title.into();
    }

    /// Return the unique URI of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// assert_eq!(entry.id(), "urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Set the unique URI of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6");
    /// ```
    pub fn set_id<V>(&mut self, id: V)
        where V: Into<String>
    {
        self.id = id.into();
    }

    /// Return the last time that this entry was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_updated("2017-06-03T15:15:44-05:00");
    /// assert_eq!(entry.updated(), "2017-06-03T15:15:44-05:00");
    /// ```
    pub fn updated(&self) -> &str {
        self.updated.as_str()
    }

    /// Set the last time that this entry was modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_updated("2017-06-03T15:15:44-05:00");
    /// ```
    pub fn set_updated<V>(&mut self, updated: V)
        where V: Into<String>
    {
        self.updated = updated.into();
    }

    /// Return the authors of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_authors(vec![Person::default()]);
    /// assert_eq!(entry.authors().len(), 1);
    /// ```
    pub fn authors(&self) -> &[Person] {
        self.authors.as_slice()
    }

    /// Set the authors of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_authors(vec![Person::default()]);
    /// ```
    pub fn set_authors<V>(&mut self, authors: V)
        where V: Into<Vec<Person>>
    {
        self.authors = authors.into();
    }

    /// Return the categories this entry belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Category};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_categories(vec![Category::default()]);
    /// assert_eq!(entry.categories().len(), 1);
    /// ```
    pub fn categories(&self) -> &[Category] {
        self.categories.as_slice()
    }

    /// Set the categories this entry belongs to.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Category};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_categories(vec![Category::default()]);
    /// ```
    pub fn set_categories<V>(&mut self, categories: V)
        where V: Into<Vec<Category>>
    {
        self.categories = categories.into();
    }

    /// Return the contributors to this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_contributors(vec![Person::default()]);
    /// assert_eq!(entry.contributors().len(), 1);
    /// ```
    pub fn contributors(&self) -> &[Person] {
        self.contributors.as_slice()
    }

    /// Set the contributors to this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Person};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_contributors(vec![Person::default()]);
    /// ```
    pub fn set_contributors<V>(&mut self, contributors: V)
        where V: Into<Vec<Person>>
    {
        self.contributors = contributors.into();
    }

    /// Return the links for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Link};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_links(vec![Link::default()]);
    /// assert_eq!(entry.links().len(), 1);
    /// ```
    pub fn links(&self) -> &[Link] {
        self.links.as_slice()
    }

    /// Set the links for this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Link};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_links(vec![Link::default()]);
    /// ```
    pub fn set_links<V>(&mut self, links: V)
        where V: Into<Vec<Link>>
    {
        self.links = links.into();
    }

    /// Return the time that this entry was initially created or first made available.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_published("2017-06-01T15:15:44-05:00".to_string());
    /// assert_eq!(entry.published(), Some("2017-06-01T15:15:44-05:00"));
    /// ```
    pub fn published(&self) -> Option<&str> {
        self.published.as_ref().map(|s| s.as_str())
    }

    /// Set the time that this entry was initially created or first made available.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_published("2017-06-01T15:15:44-05:00".to_string());
    /// ```
    pub fn set_published<V>(&mut self, published: V)
        where V: Into<Option<String>>
    {
        self.published = published.into();
    }

    /// Return the source of this entry if it was copied from another feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Source};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_source(Source::default());
    /// assert!(entry.source().is_some());
    /// ```
    pub fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    /// Set the source of this entry if it was copied from another feed.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Source};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_source(Source::default());
    /// ```
    pub fn set_source<V>(&mut self, source: V)
        where V: Into<Option<Source>>
    {
        self.source = source.into()
    }

    /// Return the summary of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_summary("Entry summary.".to_string());
    /// assert_eq!(entry.summary(), Some("Entry summary."));
    /// ```
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.as_str())
    }

    /// Set the summary of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_summary("Entry summary.".to_string());
    /// ```
    pub fn set_summary<V>(&mut self, summary: V)
        where V: Into<Option<String>>
    {
        self.summary = summary.into();
    }

    /// Return the information about the rights held in and over this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_rights("© 2017 John Doe".to_string());
    /// assert_eq!(entry.rights(), Some("© 2017 John Doe"));
    /// ```
    pub fn rights(&self) -> Option<&str> {
        self.rights.as_ref().map(|s| s.as_str())
    }

    /// Set the information about the rights held in and over this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::Entry;
    ///
    /// let mut entry = Entry::default();
    /// entry.set_rights("© 2017 John Doe".to_string());
    /// ```
    pub fn set_rights<V>(&mut self, rights: V)
        where V: Into<Option<String>>
    {
        self.rights = rights.into();
    }

    /// Return the content of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Content};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_content(Content::default());
    /// assert!(entry.content().is_some());
    /// ```
    pub fn content(&self) -> Option<&Content> {
        self.content.as_ref()
    }

    /// Set the content of this entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use atom_syndication::{Entry, Content};
    ///
    /// let mut entry = Entry::default();
    /// entry.set_content(Content::default());
    /// assert!(entry.content().is_some());
    /// ```
    pub fn set_content<V>(&mut self, content: V)
        where V: Into<Option<Content>>
    {
        self.content = content.into();
    }
}

impl FromXml for Entry {
    fn from_xml<B: BufRead>(reader: &mut Reader<B>, _: Attributes) -> Result<Self, Error> {
        let mut entry = Entry::default();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(element) => {
                    match element.name() {
                        b"id" => entry.id = atom_text(reader)?.unwrap_or_default(),
                        b"title" => entry.title = atom_text(reader)?.unwrap_or_default(),
                        b"updated" => entry.updated = atom_text(reader)?.unwrap_or_default(),
                        b"author" => {
                            entry
                                .authors
                                .push(Person::from_xml(reader, element.attributes())?)
                        }
                        b"category" => {
                            entry
                                .categories
                                .push(Category::from_xml(reader, element.attributes())?)
                        }
                        b"contributor" => {
                            entry
                                .contributors
                                .push(Person::from_xml(reader, element.attributes())?)
                        }
                        b"link" => {
                            entry
                                .links
                                .push(Link::from_xml(reader, element.attributes())?)
                        }
                        b"published" => entry.published = atom_text(reader)?,
                        b"source" => {
                            entry.source = Some(Source::from_xml(reader, element.attributes())?)
                        }
                        b"summary" => entry.summary = atom_text(reader)?,
                        b"rights" => entry.rights = atom_text(reader)?,
                        b"content" => {
                            entry.content = Some(Content::from_xml(reader, element.attributes())?)
                        }
                        n => reader.read_to_end(n, &mut Vec::new())?,
                    }
                }
                Event::End(_) => break,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }

            buf.clear();
        }

        Ok(entry)
    }
}