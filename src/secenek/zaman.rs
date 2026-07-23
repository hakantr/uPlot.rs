/// uPlot `fmtDate(tpl, names)` içindeki yerelleştirilebilir tarih adları.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TarihAdları {
    pub uzun_aylar: [String; 12],
    pub kısa_aylar: [String; 12],
    pub uzun_hafta_günleri: [String; 7],
    pub kısa_hafta_günleri: [String; 7],
}

impl TarihAdları {
    pub fn yeni(
        uzun_aylar: [String; 12],
        kısa_aylar: [String; 12],
        uzun_hafta_günleri: [String; 7],
        kısa_hafta_günleri: [String; 7],
    ) -> Self {
        Self {
            uzun_aylar,
            kısa_aylar,
            uzun_hafta_günleri,
            kısa_hafta_günleri,
        }
    }

    pub fn ingilizce() -> Self {
        Self::yeni(
            [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]
            .map(str::to_string),
            [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]
            .map(str::to_string),
            [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ]
            .map(str::to_string),
            ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map(str::to_string),
        )
    }

    pub fn rusça() -> Self {
        Self::yeni(
            [
                "Январь",
                "Февраль",
                "Март",
                "Апрель",
                "Май",
                "Июнь",
                "Июль",
                "Август",
                "Сентябрь",
                "Октябрь",
                "Ноябрь",
                "Декабрь",
            ]
            .map(str::to_string),
            [
                "Янв", "Февр", "Март", "Апр", "Май", "Июнь", "Июль", "Авг", "Сент", "Окт", "Нояб",
                "Дек",
            ]
            .map(str::to_string),
            [
                "Воскресенье",
                "Понедельник",
                "Вторник",
                "Среда",
                "Четверг",
                "Пятница",
                "Суббота",
            ]
            .map(str::to_string),
            ["Вск", "Пнд", "Втр", "Срд", "Чтв", "Птн", "Сбт"].map(str::to_string),
        )
    }

    pub fn kısa_ay(&self, ay: u32) -> Option<&str> {
        let indeks = usize::try_from(ay.checked_sub(1)?).ok()?;
        self.kısa_aylar.get(indeks).map(String::as_str)
    }
}

impl Default for TarihAdları {
    fn default() -> Self {
        Self::ingilizce()
    }
}
