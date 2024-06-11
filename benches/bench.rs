#![feature(test)]

extern crate test;

mod polynomial;

use crate::polynomial::{PolyRatio, Polynomial};

use polynomial::Term;
use polynomial::Variable;
use test::Bencher;

#[bench]
fn bench_polynomial_add(b: &mut Bencher) {
    let p1 = Polynomial {
        terms: vec![
            Term {
                coefficient: 1.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "y".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 3.into(),
                variables: vec![Variable {
                    name: "z".to_string(),
                    degree: 1.into(),
                }],
            },
        ],
        degree: 3.into(),
    };
    let p2 = Polynomial {
        terms: vec![
            Term {
                coefficient: 1.into(),
                variables: vec![Variable {
                    name: "a".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "b".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 3.into(),
                variables: vec![Variable {
                    name: "c".to_string(),
                    degree: 4.into(),
                }],
            },
        ],
        degree: 1.into(),
    };
    b.iter(|| {
        let _ = p1.clone() + p2.clone();
    });
}

#[bench]
fn bench_polynomial_mul(b: &mut Bencher) {
    let p1 = Polynomial {
        terms: vec![
            Term {
                coefficient: 1.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "y".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 3.into(),
                variables: vec![Variable {
                    name: "z".to_string(),
                    degree: 1.into(),
                }],
            },
        ],
        degree: 3.into(),
    };
    let p2 = Polynomial {
        terms: vec![
            Term {
                coefficient: 1.into(),
                variables: vec![Variable {
                    name: "a".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "b".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 3.into(),
                variables: vec![Variable {
                    name: "c".to_string(),
                    degree: 4.into(),
                }],
            },
        ],
        degree: 1.into(),
    };
    b.iter(|| {
        let _ = p1.clone() * p2.clone();
    });
}

#[bench]
fn bench_polynomial_div(b: &mut Bencher) {
    let p1 = Polynomial {
        terms: vec![
            Term {
                coefficient: 8.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "y".to_string(),
                    degree: 1.into(),
                }],
            },
        ],
        degree: 1.into(),
    };
    let p2 = Polynomial {
        terms: vec![Term {
            coefficient: 2.into(),
            variables: vec![Variable {
                name: "x".to_string(),
                degree: 1.into(),
            }],
        }],
        degree: 1.into(),
    };
    b.iter(|| {
        let _ = p1.clone() / p2.clone();
    });
}

#[bench]
fn bench_polynomial_roots_linear(b: &mut Bencher) {
    let p = Polynomial {
        terms: vec![
            Term {
                coefficient: 3.into(),
                variables: vec![
                    Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    },
                    Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    },
                ],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "z".to_string(),
                    degree: 1.into(),
                }],
            },
        ],
        degree: 1.into(),
    };
    b.iter(|| {
        let _ = p.clone().roots("x");
    });
}

#[bench]
fn bench_polynomial_roots_quadratic(b: &mut Bencher) {
    let p = Polynomial {
        terms: vec![
            Term {
                coefficient: 3.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 2.into(),
                }],
            },
            Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            },
            Term {
                coefficient: 1.into(),
                variables: vec![
                    Variable {
                        name: "x".to_string(),
                        degree: 0.into(),
                    },
                    Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    },
                ],
            },
        ],
        degree: 1.into(),
    };
    b.iter(|| {
        let _ = p.clone().roots("x");
    });
}

#[bench]
fn bench_polyratio_add(b: &mut Bencher) {
    let p1 = PolyRatio {
        numerator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 3.into(),
                    variables: vec![Variable {
                        name: "z".to_string(),
                        degree: 1.into(),
                    }],
                },
            ],
            degree: 3.into(),
        },
        denominator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "a".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "b".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 3.into(),
                    variables: vec![Variable {
                        name: "c".to_string(),
                        degree: 4.into(),
                    }],
                },
            ],
            degree: 1.into(),
        },
    };
    let p2 = PolyRatio {
        numerator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 3.into(),
                    variables: vec![Variable {
                        name: "z".to_string(),
                        degree: 1.into(),
                    }],
                },
            ],
            degree: 3.into(),
        },
        denominator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "a".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "b".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 3.into(),
                    variables: vec![Variable {
                        name: "c".to_string(),
                        degree: 4.into(),
                    }],
                },
            ],
            degree: 1.into(),
        },
    };
    b.iter(|| {
        let _ = p1.clone() + p2.clone();
    });
}

#[bench]
fn bench_polyratio_div(b: &mut Bencher) {
    let p1 = PolyRatio {
        numerator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    }],
                },
            ],
            degree: 1.into(),
        },
        denominator: Polynomial {
            terms: vec![Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            }],
            degree: 1.into(),
        },
    };
    let p2 = PolyRatio {
        numerator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    }],
                },
            ],
            degree: 1.into(),
        },
        denominator: Polynomial {
            terms: vec![Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            }],
            degree: 1.into(),
        },
    };
    b.iter(|| {
        let _ = p1.clone() / p2.clone();
    });
}

#[bench]
fn bench_polyratio_simplify(b: &mut Bencher) {
    let p = PolyRatio {
        numerator: Polynomial {
            terms: vec![
                Term {
                    coefficient: 1.into(),
                    variables: vec![Variable {
                        name: "x".to_string(),
                        degree: 1.into(),
                    }],
                },
                Term {
                    coefficient: 2.into(),
                    variables: vec![Variable {
                        name: "y".to_string(),
                        degree: 1.into(),
                    }],
                },
            ],
            degree: 1.into(),
        },
        denominator: Polynomial {
            terms: vec![Term {
                coefficient: 2.into(),
                variables: vec![Variable {
                    name: "x".to_string(),
                    degree: 1.into(),
                }],
            }],
            degree: 1.into(),
        },
    };
    b.iter(|| {
        let _ = p.clone().simplify();
    });
}
