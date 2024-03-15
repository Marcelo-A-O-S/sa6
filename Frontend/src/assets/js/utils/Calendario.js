const isLeapYear = (year) => {
  return (
    (year % 4 === 0 && year % 100 !== 0) || (year % 400 === 0)
  );
};

const getFebDays = (year) => {
  return isLeapYear(year) ? 29 : 28;
};

const selecionarDia = () => {
  let dias = document.querySelectorAll('.calendar-days .day');
  dias.forEach(day => {
    day.addEventListener('click', function() {
      dias.forEach(d => d.classList.remove('selected'));
      this.classList.add('selected');
    });
  });
};

const generateCalendar = (month, year) => {
  let calendar_days = document.querySelector('.calendar-days');
  calendar_days.innerHTML = '';
  let calendar_header_year = document.querySelector('#year');
  let days_of_month = [
    31, getFebDays(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
  ];

  let currentDate = new Date();

  const month_picker = document.querySelector('#month-picker'); // Mover para dentro da função generateCalendar()

  month_picker.innerHTML = month_names[month];

  calendar_header_year.innerHTML = year;

  let first_day = new Date(year, month);

  for (let i = 0; i <= days_of_month[month] + first_day.getDay() - 1; i++) {
    let day = document.createElement('div');
    day.classList.add('day'); // Adiciona a classe 'day'

    if (i >= first_day.getDay()) {
      day.innerHTML = i - first_day.getDay() + 1;

<<<<<<< HEAD
      if (i - first_day.getDay() + 1 === currentDate.getDate() &&
        year === currentDate.getFullYear() &&
        month === currentDate.getMonth()
      ) {
        day.classList.add('current-date');
=======
    for (let i = 0; i <= days_of_month[month] + first_day.getDay() - 1; i++) {
      let day = document.createElement('div');
      day.classList.add("day")

      if (i >= first_day.getDay()) {
        day.innerHTML = i - first_day.getDay() + 1;

        if (i - first_day.getDay() + 1 === currentDate.getDate() &&
          year === currentDate.getFullYear() &&
          month === currentDate.getMonth()
        ) {
          day.classList.add('current-date');

        }
>>>>>>> c12f0eaf856483ef89a603daebd5c3541b3c8677
      }
    }
    calendar_days.appendChild(day);
  }

  // Após gerar os dias do calendário, chame a função para selecionar o dia
  selecionarDia();

  const month_list = document.querySelector('.month-list'); // Mover para dentro da função generateCalendar()

  month_names.forEach((e, index) => {
    let month = document.createElement('div');
    month.innerHTML = `<div>${e}</div>`;

    month_list.append(month);
    month.onclick = () => {
      currentMonth.value = index;
      generateCalendar(currentMonth.value, currentYear.value);
      month_list.classList.replace('show', 'hide');
      dayTextFormat.classList.remove('hideTime');
      dayTextFormat.classList.add('showtime');
      timeFormat.classList.remove('hideTime');
      timeFormat.classList.add('showtime');
      dateFormat.classList.remove('hideTime');
      dateFormat.classList.add('showtime');
    };
  });
};

// Variáveis para o seletor de mês e outros elementos
const month_names = [
  'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho', 'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
];
const dayTextFormat = document.querySelector('.day-text-format');
const timeFormat = document.querySelector('.time-format');
const dateFormat = document.querySelector('.date-format');
const calendar = document.querySelector('.calendar');
const currentMonth = { value: new Date().getMonth() };
const currentYear = { value: new Date().getFullYear() };

generateCalendar(currentMonth.value, currentYear.value);

// Evento de clique para o seletor de mês
const month_picker = document.querySelector('#month-picker');
const month_list = document.querySelector('.month-list');

month_picker.onclick = () => {
  month_list.classList.remove('hideonce');
  month_list.classList.remove('hide');
  month_list.classList.add('show');
  dayTextFormat.classList.remove('showtime');
  dayTextFormat.classList.add('hidetime');
  timeFormat.classList.remove('showtime');
  timeFormat.classList.add('hideTime');
  dateFormat.classList.remove('showtime');
  dateFormat.classList.add('hideTime');
};

// Eventos para navegar entre os anos
document.querySelector('#pre-year').onclick = () => {
  --currentYear.value;
  generateCalendar(currentMonth.value, currentYear.value);
};
document.querySelector('#next-year').onclick = () => {
  ++currentYear.value;
  generateCalendar(currentMonth.value, currentYear.value);
};


