languages = (
    'markdown',
    'article',  # my article
)


class AST(object):
    def __init__(self, language):
        if language not in languages:
            raise ValueError("Unknown language: {}".format(language))
        self.language = language
        self.tree = tuple()

    def load(self, s):
        self.src = s
        MARKDOWN = 'markdown' == self.language
        if MARKDOWN:
            print('TODO: implement Markdown')
        else:
            from . import article_ast
            self.tree = article_ast(self.src)
            return self.tree
