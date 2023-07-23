create table book (
    id serial primary key,
    title varchar not null,
    isbn varchar not null,
    author varchar not null, 
    publication_year int not null,   
    total_copies int not null,
    available_copies int not null
);

insert into book (title, isbn, author, publication_year, total_copies, available_copies) values ("Harry Potter and the Philosophers Stone", "978-0-7475-3269-9", "J. K. Rowling", 1997, 100, 100);
insert into book (title, isbn, author, publication_year, total_copies, available_copies) values ("The Alchemist", "0-06-250217-4", "Paulo Coelho", 1988, 100, 100);


