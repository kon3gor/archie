pub struct Config {
    criteria: Criteria,
    func: Func,
    value: str,
}

enum Criteria {
    ModificationDate,
    CreationDate,
    VisitDate,
    Regex,
}

enum Func {
    Greater,
    Less,
    Matches,
}
