



enum Temtem {
	// TODO
}

enum Trait {
	// TODO
}

enum Type {
	// TODO
}

enum Technique {
	// TODO
}

enum Gear {
	// TODO
}

enum TemtemType {
	Single(Type),
	Double(Type, Type),
}

struct Stats {
	hp:    u32,
	sta:   u32,
	spd:   u32,
	atk:   u32,
	def:   u32,
	spatk: u32,
	spdef: u32,
}

struct TemtemData {
	temtem:         Temtem,
	temtem_type:    TemtemType,
	base_stats:     Stats,
	trait_pool:     Vec<Trait>,
	technique_pool: Vec<Technique>,
}

struct TemtemBuild {
	data:         TemtemData,
	svs:          Stats,
	tvs:          Stats,
	stats:        Stats,
	temtem_trait: Trait,
	techniques:   [Techniques; 4],
	gear:         Option<Gear>,
}

struct StatsChange {
	spd:   i32,
	atk:   i32,
	def:   i32,
	spatk: i32,
	spdef: i32,
}

enum Condition {
	// TODO
}

struct TechniqueInBattle {
	hold: u32,
}

enum TemtemCondition {
	None,
	One(Condition, u32),
	Two {
		f: (Condition, u32),
		s: (Condition, u32),
	}
}

struct TemtemInBattle {
	build:        TemtemBuild,
	stats:        Stats,
	temtem_type:  TemtemType,
	stats_change: StatsChange,
	conditions:   TemtemCondition,
	techniques:   [TechniqueInBattle; 4],
	overexerted:  bool,
}


