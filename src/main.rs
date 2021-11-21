use lucene::store::Directory;

fn main() {
    let dir = Directory::open(
        "/home/yan/elasticsearch-7.15.2/data/nodes/0/indices/IjA0jYRrSICcD0MrR-K5tA/0/index/",
    )
    .unwrap();
}
