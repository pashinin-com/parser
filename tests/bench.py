# coding: utf-8

import os
import time
import functools


class benchmark(object):
    suites = []

    def __init__(self, name):
        self._name = name

    def __call__(self, func):
        @functools.wraps(func)
        def wrapper(text, loops=1000):
            start = time.clock()
            while loops:
                func(text)
                loops -= 1
            end = time.clock()
            return end - start
        # register
        benchmark.suites.append((self._name, wrapper))
        return wrapper

    @classmethod
    def bench(cls, text, loops=100):
        print('Markdown parsing and rendering (%d times):' % loops)
        for name, func in cls.suites:
            try:
                total = func(text, loops=loops)
                print('{0}: {1}s'.format(name, total))
            except ImportError:
                print('{0} is not available'.format(name))


@benchmark('rparser')
def benchmark_rparser(text):
    from rparser import Markdown
    m = Markdown(text)
    m.render()


@benchmark('mistune')
def benchmark_mistune(text):
    import mistune
    mistune.markdown(text)


if __name__ == '__main__':
    root = os.path.dirname(__file__)
    filepath = os.path.join(
        root, 'fixtures/normal', 'markdown_documentation_syntax.text'
    )
    with open(filepath, 'r') as f:
        text = f.read()

    benchmark.bench(text)
