create day:
    cargo generate --path ./daily-template --name {{day}}

flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package {{day}} --bin {{part}} -o flamegraphs/{{day}}--{{part}}.svg
