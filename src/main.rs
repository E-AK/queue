struct Queue<T> {
    /// Вектор (Vec), который содержит элементы очереди.
    queue: Vec<T>,
}

impl<T> Queue<T> {
    /// Создает новый экземпляр очереди.
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    /// Создает новый экземпляр очереди.
    fn length(&self) -> usize {
        self.queue.len()
    }

    /// Добавляет элемент в конец очереди.
    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    /// Удаляет и возвращает первый элемент из очереди.
    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    /// Удаляет и возвращает первый элемент из очереди.
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Возвращает ссылку на первый элемент очереди, если он существует.
    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

fn main() {
    // Создание новой очереди.
    let mut queue = Queue::new();

    // Добавление элемента 1 в очередь.
    queue.enqueue(1);

    // Добавление элемента 2 в очередь.
    queue.enqueue(2);

    // Проверка длины очереди (ожидается 2).
    assert_eq!(queue.length(), 2);

    // Извлечение и проверка извлеченного элемента (ожидается 1).
    assert_eq!(queue.dequeue(), 1);

    // Просмотр первого элемента очереди (ожидается Some(&2)).
    assert_eq!(queue.peek(), Some(&2));

    // Извлечение еще одного элемента из очереди.

    queue.dequeue();

    // Проверка, что очередь пуста после извлечения всех элементов.
    assert!(queue.is_empty());
}
