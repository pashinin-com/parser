# import os

from rparser import article_render as render

def test_normal():
    src = 'asd'
    html, info = render(src)
    assert '<p>asd</p>' == html
