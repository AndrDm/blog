use rand::Rng;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
struct Line {
    is_yang: bool,
    is_old: bool,
    was_old: bool,
}

#[derive(Clone, Copy)]
struct HexInfo {
    number: usize,
    title: &'static str,
    description: &'static str,
}

const KING_WEN_ORDER: [usize; 64] = [
    2, 24, 7, 19, 15, 36, 46, 11, 16, 51, 40, 54, 62, 55, 32, 34, 8, 3, 29, 60, 39, 63, 48, 5, 45,
    17, 47, 58, 31, 49, 28, 43, 23, 27, 4, 41, 52, 22, 18, 26, 35, 21, 64, 38, 56, 30, 50, 14, 20,
    59, 6, 61, 53, 37, 57, 9, 12, 25, 44, 13, 10, 42, 33, 1,
];

const ALL_HEXAGRAMS: [HexInfo; 64] = [
    HexInfo {
        number: 1,
        title: "1. 乾 (Qián), Творчество",
        description: "Символ творящей силы. Успех достигается настойчивостью и самосовершенствованием.",
    },
    HexInfo {
        number: 2,
        title: "2. 坤 (Kūn), Исполнение",
        description: "Символ земли и восприимчивости. Мягкость и поддержка ведут к изобилию и гармонии.",
    },
    HexInfo {
        number: 3,
        title: "3. 屯 (Zhūn), Начальная трудность",
        description: "Начальные усилия в сложных условиях. Требуют терпения и внешней поддержки.",
    },
    HexInfo {
        number: 4,
        title: "4. 蒙 (Méng), Недоразвитость",
        description: "Юность и незрелость. Нужны наставления и учёба, чтобы избежать ошибок.",
    },
    HexInfo {
        number: 5,
        title: "5. 需 (Xū), Ожидание",
        description: "Терпеливое ожидание подходящего момента. Спокойная подготовка к действию.",
    },
    HexInfo {
        number: 6,
        title: "6. 訟 (Sòng), Тяжба",
        description: "Спор и разногласия. Следует найти справедливое решение и избегать пустой конфронтации.",
    },
    HexInfo {
        number: 7,
        title: "7. 師 (Shī), Войско",
        description: "Организованная коллективная сила. Дисциплина и ясная цель ведут к успеху.",
    },
    HexInfo {
        number: 8,
        title: "8. 比 (Bǐ), Приближение",
        description: "Единство и доверие. Способствует сплочению и общим целям.",
    },
    HexInfo {
        number: 9,
        title: "9. 小畜 (Xiǎo Chù), Малая сила",
        description: "Мягкое, постепенное влияние. Требует скромности и упорства.",
    },
    HexInfo {
        number: 10,
        title: "10. 履 (Lǚ), Наступление",
        description: "Осторожный шаг вперёд по тонкой грани. Нужно удерживать равновесие.",
    },
    HexInfo {
        number: 11,
        title: "11. 泰 (Tài), Расцвет",
        description: "Гармония и процветание. Силы неба и земли соединяются.",
    },
    HexInfo {
        number: 12,
        title: "12. 否 (Pǐ), Упадок",
        description: "Противоположность расцвету: застой. Следует укрепить внутренний стержень.",
    },
    HexInfo {
        number: 13,
        title: "13. 同人 (Tóng Rén), Единомышленники",
        description: "Объединение вокруг общей цели, приводит к успеху через взаимопонимание.",
    },
    HexInfo {
        number: 14,
        title: "14. 大有 (Dà Yǒu), Обладание великим",
        description: "Большие ресурсы и влияние. Нужно использовать их мудро и скромно.",
    },
    HexInfo {
        number: 15,
        title: "15. 謙 (Qiān), Смирение",
        description: "Сила скромности и сдержанности. Признак истинной внутренней силы.",
    },
    HexInfo {
        number: 16,
        title: "16. 豫 (Yù), Вольность",
        description: "Воодушевление и радостный подъём. Стоит использовать энергию на благое дело.",
    },
    HexInfo {
        number: 17,
        title: "17. 隨 (Suí), Следование",
        description: "Приспособление к переменам, не теряя собственных корней.",
    },
    HexInfo {
        number: 18,
        title: "18. 蠱 (Gǔ), Исправление порчи",
        description: "Устранение старых ошибок ради обновления и роста.",
    },
    HexInfo {
        number: 19,
        title: "19. 臨 (Lín), Посещение",
        description: "Возрастание влияния, покровительство. Будьте справедливы и снисходительны.",
    },
    HexInfo {
        number: 20,
        title: "20. 観 (Guān), Созерцание",
        description: "Вглядеться в суть, понять причины. Спокойное наблюдение рождает мудрость.",
    },
    HexInfo {
        number: 21,
        title: "21. 噬嗑 (Shì Kè), Стиснутые зубы",
        description: "Решительное устранение препятствий и несправедливости.",
    },
    HexInfo {
        number: 22,
        title: "22. 賁 (Bì), Убранство",
        description: "Внешняя красота. Однако внутренняя суть важнее поверхностного блеска.",
    },
    HexInfo {
        number: 23,
        title: "23. 剥 (Bō), Бо",
        description: "Постепенное «слущивание» старого. В этой гексаграмме верхняя линия сплошная, а остальные – разрывные.",
    },
    HexInfo {
        number: 24,
        title: "24. 复 (Fù), Возврат",
        description: "Возвращение к истокам. Новый цикл, время пересмотреть пройденный путь.",
    },
    HexInfo {
        number: 25,
        title: "25. 无妄 (Wú Wàng), Непорочность",
        description: "Чистые мотивы без корысти. Искренность и прямота приносят удачу.",
    },
    HexInfo {
        number: 26,
        title: "26. 大畜 (Dà Chù), Воспитание великим",
        description: "Накопление силы и мудрости. Самодисциплина перед рывком.",
    },
    HexInfo {
        number: 27,
        title: "27. 頤 (Yí), Питание",
        description: "Забота о пище тела и духа. Следите за тем, что потребляете и чем делитесь.",
    },
    HexInfo {
        number: 28,
        title: "28. 大過 (Dà Guò), Переразвитие великого",
        description: "Чрезмерная нагрузка грозит поломкой. Укрепите слабые места и ослабьте избыточное.",
    },
    HexInfo {
        number: 29,
        title: "29. 坎 (Kǎn), Бездна",
        description: "Повторная опасность. Сохраняйте стойкость и веру в правильность пути.",
    },
    HexInfo {
        number: 30,
        title: "30. 離 (Lí), Сияние",
        description: "Яркость и пламя. Огонь требует постоянного питания, разум — осознанности.",
    },
    HexInfo {
        number: 31,
        title: "31. 咸 (Xián), Взаимодействие",
        description: "Притяжение сил, создающее союз. Важна искренность и взаимное уважение.",
    },
    HexInfo {
        number: 32,
        title: "32. 恒 (Héng), Постоянство",
        description: "Длительная стабильность. Верность принципам приводит к твёрдому результату.",
    },
    HexInfo {
        number: 33,
        title: "33. 遯 (Dùn), Бегство",
        description: "Временное отступление, чтобы сохранить силы и дождаться лучшего часа.",
    },
    HexInfo {
        number: 34,
        title: "34. 大壯 (Dà Zhuàng), Великая мощь",
        description: "Проявление силы и решимости. Действуйте этично и осмотрительно.",
    },
    HexInfo {
        number: 35,
        title: "35. 晉 (Jìn), Восход",
        description: "Поступательное продвижение, подобно восходу солнца. Используйте момент роста.",
    },
    HexInfo {
        number: 36,
        title: "36. 明夷 (Míng Yí), Поражение света",
        description: "Свет в тени внешних обстоятельств. Иногда лучше скрывать достоинства.",
    },
    HexInfo {
        number: 37,
        title: "37. 家人 (Jiā Rén), Домашние",
        description: "Порядок и взаимная поддержка в семье (или коллективе).",
    },
    HexInfo {
        number: 38,
        title: "38. 睽 (Kuí), Разлад",
        description: "Расхождение взглядов. Может привести к конфликту или стимулировать поиск нового.",
    },
    HexInfo {
        number: 39,
        title: "39. 蹇 (Jiǎn), Препятствие",
        description: "Трудности требуют упорства и помощи друзей. Преодолев преграду, обретаете силу.",
    },
    HexInfo {
        number: 40,
        title: "40. 解 (Xiè), Разрешение",
        description: "Освобождение от уз и проблем. Необходимо решительное действие.",
    },
    HexInfo {
        number: 41,
        title: "41. 損 (Sǔn), Убыль",
        description: "Добровольное сокращение внешнего ради укрепления внутреннего.",
    },
    HexInfo {
        number: 42,
        title: "42. 益 (Yì), Приумножение",
        description: "Увеличение потенциала. Делитесь благом — оно приумножается.",
    },
    HexInfo {
        number: 43,
        title: "43. 夬 (Guài), Выход",
        description: "Решительный прорыв. Чёткое заявление позиции для устранения препятствий.",
    },
    HexInfo {
        number: 44,
        title: "44. 姤 (Gòu), Столкновение",
        description: "Неожиданная встреча или влияние. Будьте бдительны, чтобы не допустить хаоса.",
    },
    HexInfo {
        number: 45,
        title: "45. 萃 (Cuì), Воссоединение",
        description: "Сбор людей и ресурсов для общей цели. Искреннее объединение рождает успех.",
    },
    HexInfo {
        number: 46,
        title: "46. 升 (Shēng), Подъём",
        description: "Постепенное восхождение. Шаг за шагом продвигайтесь к вершине.",
    },
    HexInfo {
        number: 47,
        title: "47. 困 (Kùn), Истощение",
        description: "Ситуация ограничений и трудностей. Проверьте внутренние ресурсы и не теряйте дух.",
    },
    HexInfo {
        number: 48,
        title: "48. 井 (Jǐng), Колодец",
        description: "Источник жизни, требующий обновления. Общая опора для многих.",
    },
    HexInfo {
        number: 49,
        title: "49. 革 (Gé), Смена",
        description: "Революционные перемены. Старое изжило себя, наступает время обновления.",
    },
    HexInfo {
        number: 50,
        title: "50. 鼎 (Dǐng), Жертвенник",
        description: "Символ котла и преобразования. Высшая трансформация и духовная пища.",
    },
    HexInfo {
        number: 51,
        title: "51. 震 (Zhèn), Возбуждение",
        description: "Гром, внезапный толчок. Пробуждает и заставляет действовать.",
    },
    HexInfo {
        number: 52,
        title: "52. 艮 (Gèn), Сосредоточенность",
        description: "Спокойствие горы. Умение остановиться и вглядеться в себя.",
    },
    HexInfo {
        number: 53,
        title: "53. 漸 (Jiàn), Постепенность",
        description: "Медленный, но верный рост, подобно созреванию плода. Требует времени.",
    },
    HexInfo {
        number: 54,
        title: "54. 歸妹 (Guī Mèi), Невеста",
        description: "Ситуация, где нужно принимать условия. Помните об этике и гармонии.",
    },
    HexInfo {
        number: 55,
        title: "55. 豐 (Fēng), Изобилие",
        description: "Пик процветания и яркости. Важно не забывать о мере и сути.",
    },
    HexInfo {
        number: 56,
        title: "56. 旅 (Lǚ), Странствие",
        description: "Путешествие и жизнь вдали от дома. Сохраняйте осторожность и достоинство.",
    },
    HexInfo {
        number: 57,
        title: "57. 巽 (Xùn), Проникновение",
        description: "Мягкая сила ветра. Деликатное воздействие способно проникать глубоко.",
    },
    HexInfo {
        number: 58,
        title: "58. 兑 (Duì), Радость",
        description: "Искренняя открытость и весёлое общение. Главное — не стать поверхностным.",
    },
    HexInfo {
        number: 59,
        title: "59. 涣 (Huàn), Рассеяние",
        description: "Разуплотнение старых форм. Освобождение от оков и объединение новых сил.",
    },
    HexInfo {
        number: 60,
        title: "60. 節 (Jié), Ограничение",
        description: "Разумные границы и правила. Помогают сохранить энергию и фокус.",
    },
    HexInfo {
        number: 61,
        title: "61. 中孚 (Zhōng Fú), Внутренняя правда",
        description: "Искренность сердца создает доверие и гармонию в отношениях.",
    },
    HexInfo {
        number: 62,
        title: "62. 小過 (Xiǎo Guò), Переразвитие малого",
        description: "Чрезмерное внимание к деталям. Нужна осторожность, чтобы не упустить главное.",
    },
    HexInfo {
        number: 63,
        title: "63. 既濟 (Jì Jì), Уже конец",
        description: "Все элементы на своих местах. Но требуется поддерживать порядок, чтобы не рухнул.",
    },
    HexInfo {
        number: 64,
        title: "64. 未濟 (Wèi Jì), Еще не конец",
        description: "Последний шаг до полного завершения. Нужна аккуратность и осознанность.",
    },
];

#[wasm_bindgen]
pub fn generate_reading_text() -> String {
    let mut rng = rand::rng();

    let original_lines = generate_hexagram(&mut rng);
    let original_number = get_king_wen_number(&original_lines);
    let original_hex = ALL_HEXAGRAMS[original_number - 1];

    let final_lines = make_stable(&original_lines);
    let final_number = get_king_wen_number(&final_lines);
    let final_hex = ALL_HEXAGRAMS[final_number - 1];

    debug_assert_eq!(original_hex.number, original_number);
    debug_assert_eq!(final_hex.number, final_number);

    let mut out = String::new();

    out.push_str("ИСХОДНАЯ ГЕКСАГРАММА:\n");
    out.push_str(&format!("King Wen = {}\n", original_number));
    out.push_str(original_hex.title);
    out.push_str("\n\n");
    out.push_str(original_hex.description);
    out.push_str("\n--------------------------------\n");
    out.push_str("РЕЗУЛЬТИРУЮЩАЯ ГЕКСАГРАММА:\n");
    out.push_str(&format!("King Wen = {}\n", final_number));
    out.push_str(final_hex.title);
    out.push_str("\n\n");
    out.push_str(final_hex.description);
    out.push_str("\n\n");

    out.push_str(&format!("ИСХОДНАЯ: {}\n", original_hex.title));
    append_lines(&mut out, &original_lines);
    out.push('\n');
    out.push_str(&format!("РЕЗУЛЬТИРУЮЩАЯ: {}\n", final_hex.title));
    append_lines(&mut out, &final_lines);

    out
}

fn append_lines(out: &mut String, lines: &[Line; 6]) {
    for line in lines {
        let (glyph, symbol, description) = line_label(*line);
        out.push_str(&format!("{}  {} ({})\n", glyph, symbol, description));
    }
}

fn generate_hexagram(rng: &mut impl Rng) -> [Line; 6] {
    let mut lines = [Line {
        is_yang: false,
        is_old: false,
        was_old: false,
    }; 6];

    for line in &mut lines {
        let mut sum = 0;
        for _ in 0..3 {
            sum += if rng.random::<bool>() { 3 } else { 2 };
        }

        *line = match sum {
            6 => Line {
                is_yang: false,
                is_old: true,
                was_old: true,
            },
            7 => Line {
                is_yang: true,
                is_old: false,
                was_old: false,
            },
            8 => Line {
                is_yang: false,
                is_old: false,
                was_old: false,
            },
            9 => Line {
                is_yang: true,
                is_old: true,
                was_old: true,
            },
            _ => unreachable!("Сумма бросков должна быть в диапазоне 6..=9"),
        };
    }

    lines
}

fn get_king_wen_number(lines: &[Line; 6]) -> usize {
    let mut index: usize = 0;

    for i in 0..6 {
        if lines[5 - i].is_yang {
            index |= 1 << i;
        }
    }

    KING_WEN_ORDER[index]
}

fn make_stable(original: &[Line; 6]) -> [Line; 6] {
    let mut result = *original;

    for line in &mut result {
        if line.is_old {
            line.is_yang = !line.is_yang;
            line.is_old = false;
        }
    }

    result
}

fn line_label(line: Line) -> (&'static str, &'static str, &'static str) {
    if line.is_yang {
        if line.was_old {
            ("━━━━━━━━━━━━", "⚊→⚋", "Старая Ян")
        } else {
            ("━━━━━━━━━━━━", "⚊", "Молодая Ян")
        }
    } else if line.was_old {
        ("━━━━━  ━━━━━", "⚋→⚊", "Старая Инь")
    } else {
        ("━━━━━  ━━━━━", "⚋", "Молодая Инь")
    }
}

#[cfg(test)]
mod tests {
    use super::generate_reading_text;

    #[test]
    fn generated_text_contains_sections() {
        let text = generate_reading_text();
        assert!(text.contains("ИСХОДНАЯ ГЕКСАГРАММА:"));
        assert!(text.contains("РЕЗУЛЬТИРУЮЩАЯ ГЕКСАГРАММА:"));
        assert!(text.contains("King Wen = "));
    }
}
