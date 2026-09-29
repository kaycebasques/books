import json
from os import environ
from pathlib import Path
import tomllib
import sys


class Book:

    def __init__(self, isbn, data):
        self.isbn = isbn
        self.title = data['title']
        self.authors = data['authors'] if 'authors' in data else None
        self.sessions = []
        for key in data:
            if key == 'title' or key == 'authors':
                continue
            self.sessions.append(Session(key, data[key]))

    def to_json(self):
        return {
            'isbn': self.isbn,
            'title': self.title,
            'authors': self.authors,
            'sessions': self.sessions
        }

class Session:

    def __init__(self, maybe_start, data):
        self.start = maybe_start if maybe_start.isdigit() else None
        self.end = data['end'] if 'end' in data else None
        self.notes = data['notes'] if 'notes' in data else None
        self.rating = data['rating'] if 'rating' in data else None
        self.progress = data['progress'] if 'progress' in data else None

    def to_json(self):
        return {
            'start': self.start,
            'end': self.end,
            'notes': self.notes,
            'rating': self.rating,
            'progress': self.progress
        }


class Encoder(json.JSONEncoder):

    def default(self, obj):
        if hasattr(obj, 'to_json'):
            return obj.to_json()
        return super().default(obj)


input_path = Path(environ['BUILD_WORKSPACE_DIRECTORY']) / 'data.toml'
data = tomllib.load(input_path.open('rb'))
books = []
for isbn in data:
    books.append(Book(isbn, data[isbn]))
output_path = Path(environ['BUILD_WORKSPACE_DIRECTORY']) / 'data.json'
output_path.write_text(json.dumps(books, cls=Encoder, indent=2))
