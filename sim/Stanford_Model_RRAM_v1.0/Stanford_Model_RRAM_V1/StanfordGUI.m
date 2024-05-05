function varargout = StanfordGUI(varargin)
% STANFORDGUI MATLAB code for StanfordGUI.fig
%      STANFORDGUI, by itself, creates a new STANFORDGUI or raises the existing
%      singleton*.
%
%      H = STANFORDGUI returns the handle to a new STANFORDGUI or the handle to
%      the existing singleton*.
%
%      STANFORDGUI('CALLBACK',hObject,eventData,handles,...) calls the local
%      function named CALLBACK in STANFORDGUI.M with the given input arguments.
%
%      STANFORDGUI('Property','Value',...) creates a new STANFORDGUI or raises the
%      existing singleton*.  Starting from the left, property value pairs are
%      applied to the GUI before StanfordGUI_OpeningFcn gets called.  An
%      unrecognized property name or invalid value makes property application
%      stop.  All inputs are passed to StanfordGUI_OpeningFcn via varargin.
%
%      *See GUI Options on GUIDE's Tools menu.  Choose "GUI allows only one
%      instance to run (singleton)".
%
% See also: GUIDE, GUIDATA, GUIHANDLES

% Edit the above text to modify the response to help StanfordGUI

% Last Modified by GUIDE v2.5 24-Jul-2018 09:35:17

% Begin initialization code - DO NOT EDIT
gui_Singleton = 1;
gui_State = struct('gui_Name',       mfilename, ...
                   'gui_Singleton',  gui_Singleton, ...
                   'gui_OpeningFcn', @StanfordGUI_OpeningFcn, ...
                   'gui_OutputFcn',  @StanfordGUI_OutputFcn, ...
                   'gui_LayoutFcn',  [] , ...
                   'gui_Callback',   []);
if nargin && ischar(varargin{1})
    gui_State.gui_Callback = str2func(varargin{1});
end

if nargout
    [varargout{1:nargout}] = gui_mainfcn(gui_State, varargin{:});
else
    gui_mainfcn(gui_State, varargin{:});
end
% End initialization code - DO NOT EDIT


% --- Executes just before StanfordGUI is made visible.
function StanfordGUI_OpeningFcn(hObject, eventdata, handles, varargin)
% This function has no output args, see OutputFcn.
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)
% varargin   command line arguments to StanfordGUI (see VARARGIN)

% Choose default command line output for StanfordGUI
handles.output = hObject;

% Update handles structure
guidata(hObject, handles);

load('Parameters.mat')

global Parameters

cla(handles.axes1,'reset'); % Deleting last graph
cla(handles.axes2,'reset'); % Deleting last graph
cla(handles.axes3,'reset'); % Deleting last graph
cla(handles.axes4,'reset'); % Deleting last graph



Slider_Gamma = Parameters.Gamma;
set(handles.edit_gamma,'String',num2str(Slider_Gamma))

Slider_Beta = Parameters.Beta;
set(handles.edit_beta,'String',num2str(Slider_Beta))

Slider_I0 = Parameters.I0;
set(handles.edit_I0,'String',num2str(Slider_I0))

Slider_g0 = Parameters.g0;
set(handles.edit_g0,'String',num2str(Slider_g0))

Slider_V0 = Parameters.V0;
set(handles.edit_V0,'String',num2str(Slider_V0))

Temp_Max = Parameters.Temp_Max;
set(handles.edit_temp_limitation_max,'String',num2str(Temp_Max))

Temp_Min = Parameters.Temp_Min;
set(handles.edit_temp_limitation_min,'String',num2str(Temp_Min))





% clear 
% clc

% UIWAIT makes StanfordGUI wait for user response (see UIRESUME)
% uiwait(handles.figure1);


% --- Outputs from this function are returned to the command line.
function varargout = StanfordGUI_OutputFcn(hObject, eventdata, handles) 
% varargout  cell array for returning output args (see VARARGOUT);
% hObject    handle to figure
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Get default command line output from handles structure
varargout{1} = handles.output;


% --- Executes on button press in pushbutton_simulate.
function pushbutton_simulate_Callback(hObject, eventdata, handles)
% hObject    handle to pushbutton_simulate (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

%%%%% rram_v_1_0_0 %%%%%

% clear
% clc

cla(handles.axes1,'reset'); % Deleting last graph
cla(handles.axes2,'reset'); % Deleting last graph
cla(handles.axes3,'reset'); % Deleting last graph
cla(handles.axes4,'reset'); % Deleting last graph


global Parameters

version                 = 1.00;
model_switch            = Parameters.Technological.Model_Value;
kb                      = 1.3806503e-23;
q                       = 1.6e-19;
g0                      = Parameters.g0;
V0                      = Parameters.V0;
Vel0                    = Parameters.Technological.Vel0;
I0                      = Parameters.I0;
beta                    = Parameters.Beta;
gamma0                  = Parameters.Gamma;
T_crit                  = Parameters.Technological.T_crit;
deltaGap0               = Parameters.Technological.Gap0;
T_smth                  = Parameters.Technological.T_smth;
Ea                      = Parameters.Physical.Ea;
a0                      = Parameters.Physical.a0;
T_ini                   = Parameters.Technological.Ini_t;
F_min                   = Parameters.Technological.Min_Field;
gap_ini                 = Parameters.Technological.Gap.Ini;
gap_min                 = Parameters.Technological.Gap.Min;
gap_max                 = Parameters.Technological.Gap.Max;
Rth                     = Parameters.Technological.Rth;
tox                     = Parameters.Technological.Tox;
rand_seed_ini           = 0;
time_step               = 0.0001; %1e-9;

pulse_width             = 20e-9;

Vtb = 0;
Itb = 0;
gap = gap_ini;

rand_seed = rand_seed_ini;

contador = 1;
contador_r = 1;
t_0 = 0;
t1 = 0;

gap_incrementos = 0;
gap_incrementos_random = 0;

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

Progress = 'Simulation in progress';
% 
% % Se crea una barra de espera para ver el avance de los ciclos...
title_bar = sprintf('%s', Progress);
wait_bar = waitbar(0,'Simulating...','CreateCancelBtn','delete(gcbf)');
set(wait_bar, 'WindowStyle','modal', 'Name', title_bar);

% Esto es para que se refresque la interfaz
drawnow;

try
    waitbar(0,wait_bar,'Simulating...');
catch
    % si no se puede actualizar la barra de espera, es que 
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end

% Se actualiza la barra de espera
try
    message = sprintf('Simulating. Please wait...');
    waitbar(0/100,wait_bar,message);
catch
    % si no se puede actualizar la barra de espera, es que
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end


% % for t = 0:0.1:1;
for Vtb = 0:0.01:2

T_cur = T_ini + abs( Vtb * Itb * Rth);
if T_cur > Parameters.Temp_Max
    T_cur = Parameters.Temp_Max;
end

Salida.T(contador)= T_cur;

t1 = t1 + time_step;

if Vtb == 0
    t1 = 0;
end

gamma_ini = gamma0;

if(Vtb < 0) 
    gamma_ini = 16;
end

gamma = gamma_ini - beta*(gap/1e-9)^3;

if ((gamma * abs(Vtb)/tox) < F_min)
    gamma = 0;
end

% calculate next time step gap situation

gap_ddt = -Vel0*exp(-q*Ea/(kb*T_cur)) * sinh((gamma*a0*q*Vtb)/(tox*kb*T_cur));

% gap time derivative - variation part

deltaGap = deltaGap0 * model_switch;
gap_random_ddt = normpdf(rand_seed,0,1) * deltaGap / (1 + exp((T_crit - T_cur)/T_smth));

%gap = idt(gap_ddt+gap_random_ddt, gap_ini);

syms t
gap_syms = int(gap_ddt,t,t_0,t1);
gap_syms = double(gap_syms);

gap_random = int(gap_random_ddt,t,t_0,t1);
gap_random = double(gap_random);

gap_incrementos = gap_syms + gap_incrementos;
gap_incrementos_random = gap_random + gap_incrementos_random;


gap = gap_ini + gap_incrementos + gap_incrementos_random;
      
        
if(gap<gap_min)
    gap = gap_min;
elseif (gap>gap_max)
    gap = gap_max;
end

Salida.Gap(contador) = gap;

Itb = I0 * exp(-gap/g0)*sinh(Vtb/V0);
Salida.Itb(contador)= Itb;

Salida.Vtb(contador) = Vtb;

%time_step = time_step + time_step;
contador = contador + 1;

t_0 = t1;
%gap_ini = gap;

end

% load test 
% 
% figure(1)
% plot(Salida.Vtb,Salida.Itb)
% set(gca, 'YScale', 'log')
% title('Vtb vs Itb')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on

% figure(2)
% plot(tiempo,current_total)
% set(gca, 'YScale', 'log')
% title('Time vs Current')
% xlabel('Time (s)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on

% figure(3)
% plot(Salida.Vtb,Salida.Gap*1e9)
% %set(gca, 'YScale', 'log')
% title('Voltage vs Gap')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (nm)') % y-axis label
% grid on
% 
% figure(4)
% plot(Salida.Vtb,Salida.T)
% set(gca, 'YScale', 'log')
% title('Voltage vs T')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on

% Se actualiza la barra de espera
try
    message = sprintf('Simulating. Please wait...');
    waitbar(25/100,wait_bar,message);
catch
    % si no se puede actualizar la barra de espera, es que
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end

%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%
% VUELTA SET
%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%

for Vtb = 2:-0.01:0

T_cur = T_ini + abs( Vtb * Itb * Rth);
if T_cur > Parameters.Temp_Max
    T_cur = Parameters.Temp_Max;
end

Salida.T(contador)= T_cur;

t1 = t1 + time_step;

if Vtb == 0
    t1 = 0;
end

gamma_ini = gamma0;

if(Vtb < 0) 
    gamma_ini = 16;
end

gamma = gamma_ini - beta*(gap/1e-9)^3;

if ((gamma * abs(Vtb)/tox) < F_min)
    gamma = 0;
end

% calculate next time step gap situation

gap_ddt = -Vel0*exp(-q*Ea/(kb*T_cur)) * sinh((gamma*a0*q*Vtb)/(tox*kb*T_cur));

% gap time derivative - variation part

deltaGap = deltaGap0 * model_switch;
gap_random_ddt = normpdf(rand_seed,0,1) * deltaGap / (1 + exp((T_crit - T_cur)/T_smth));

%gap = idt(gap_ddt+gap_random_ddt, gap_ini);

syms t
gap_syms = int(gap_ddt,t,t_0,t1);
gap_syms = double(gap_syms);

gap_random = int(gap_random_ddt,t,t_0,t1);
gap_random = double(gap_random);

gap_incrementos = gap_syms + gap_incrementos;
gap_incrementos_random = gap_random + gap_incrementos_random;


gap = gap_ini + gap_incrementos + gap_incrementos_random;
      
        
if(gap<gap_min)
    gap = gap_min;
elseif (gap>gap_max)
    gap = gap_max;
end

Salida.Gap(contador) = gap;

Itb = I0 * exp(-gap/g0)*sinh(Vtb/V0);
Salida.Itb(contador)= Itb;

Salida.Vtb(contador) = Vtb;

time_step = time_step + time_step;
contador = contador + 1;

t_0 = t1;
gap_ini = gap;

end

% figure(1)
% plot(Salida.Vtb,-Salida.Itb)
% set(gca, 'YScale', 'log')
% title('Vtb vs Itb')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on

% figure(2)
% plot(tiempo,current_total)
% set(gca, 'YScale', 'log')
% title('Time vs Current')
% xlabel('Time (s)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on
% 
% figure(3)
% plot(Salida.Vtb,Salida.Gap)
% set(gca, 'YScale', 'log')
% title('Voltage vs Gap')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on
% 
% figure(4)
% plot(Salida.Vtb,Salida.T)
% set(gca, 'YScale', 'log')
% title('Voltage vs T')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on

% Se actualiza la barra de espera
try
    message = sprintf('Simulating. Please wait...');
    waitbar(50/100,wait_bar,message);
catch
    % si no se puede actualizar la barra de espera, es que
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end

%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%
%RESET
%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%

gap_ini = 2e-10;

for Vtb = 0:-0.01:-2
    
T_cur = T_ini + abs( Vtb * Itb * Rth);
if T_cur > Parameters.Temp_Min
    T_cur = Parameters.Temp_Min;
end
Salida.T_r(contador_r)= T_cur;

t1 = t1 + time_step;

if Vtb == 0
    t1 = 0;
end

gamma_ini = gamma0;

if(Vtb < 0) 
    gamma_ini = 16;
end

gamma = gamma_ini - beta*(gap/1e-9)^3;

if ((gamma * abs(Vtb)/tox) < F_min)
    gamma = 0;
end

% calculate next time step gap situation

gap_ddt = -Vel0*exp(-q*Ea/(kb*T_cur)) * sinh((gamma*a0*q*Vtb)/(tox*kb*T_cur));

% gap time derivative - variation part

deltaGap = deltaGap0 * model_switch;
gap_random_ddt = normpdf(rand_seed,0,1) * deltaGap / (1 + exp((T_crit - T_cur)/T_smth));

syms t
gap_syms = int(gap_ddt,t,t_0,t1);
gap_syms = double(gap_syms);

gap_random = int(gap_random_ddt,t,t_0,t1);
gap_random = double(gap_random);

gap_incrementos = gap_syms + gap_incrementos;
gap_incrementos_random = gap_random + gap_incrementos_random;

gap = gap_ini + gap_incrementos + gap_incrementos_random;
         
if(gap<gap_min)
    gap = gap_min;
elseif (gap>gap_max)
    gap = gap_max;
end

Salida.Gap_r(contador_r) = gap;

Itb = I0 * exp(-gap/g0)*sinh(Vtb/V0);
Salida.Itb_r(contador_r)= Itb;

Salida.Vtb_r(contador_r) = Vtb;

%time_step = time_step + time_step;
contador_r = contador_r + 1;

t_0 = t1;
%gap_ini = gap;

end

% figure(1)
% plot(Salida.Vtb_r,-Salida.Itb_r)
% set(gca, 'YScale', 'log')
% title('Vtb vs Itb')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on
%
% figure(2)
% plot(tiempo,current_total)
% set(gca, 'YScale', 'log')
% title('Time vs Current')
% xlabel('Time (s)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on
% 
% figure(3)
% plot(Salida.Vtb_r,Salida.Gap_r)
% set(gca, 'YScale', 'log')
% title('Voltage vs Gap')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on
% 
% figure(4)
% plot(Salida.Vtb_r,Salida.T_r)
% set(gca, 'YScale', 'log')
% title('Voltage vs T')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on

% Se actualiza la barra de espera
try
    message = sprintf('Simulating. Please wait...');
    waitbar(75/100,wait_bar,message);
catch
    % si no se puede actualizar la barra de espera, es que
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end

%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%
%VUELTA RESET
%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%

gap_ini = 2e-10;

for Vtb = -2:0.001:0

T_cur = T_ini + abs( Vtb * Itb * Rth);
if T_cur > Parameters.Temp_Min
    T_cur = Parameters.Temp_Min;
end
Salida.T_r(contador_r)= T_cur;

t1 = t1 + time_step;

if Vtb == 0
    t1 = 0;
end

gamma_ini = gamma0;

if(Vtb < 0) 
    gamma_ini = 16;
end

gamma = gamma_ini - beta*(gap/1e-9)^3;

if ((gamma * abs(Vtb)/tox) < F_min)
    gamma = 0;
end

% calculate next time step gap situation

gap_ddt = -Vel0*exp(-q*Ea/(kb*T_cur)) * sinh((gamma*a0*q*Vtb)/(tox*kb*T_cur));

% gap time derivative - variation part

deltaGap = deltaGap0 * model_switch;
gap_random_ddt = normpdf(rand_seed,0,1) * deltaGap / (1 + exp((T_crit - T_cur)/T_smth));


syms t
gap_syms = int(gap_ddt,t,t_0,t1);
gap_syms = double(gap_syms);

gap_random = int(gap_random_ddt,t,t_0,t1);
gap_random = double(gap_random);

gap_incrementos = gap_syms + gap_incrementos;
gap_incrementos_random = gap_random + gap_incrementos_random;


gap = gap_ini + gap_incrementos + gap_incrementos_random;
      
        
if(gap<gap_min)
    gap = gap_min;
elseif (gap>gap_max)
    gap = gap_max;
end

Salida.Gap_r(contador_r) = gap;

Itb = I0 * exp(-gap/g0)*sinh(Vtb/V0);
Salida.Itb_r(contador_r)= Itb;

Salida.Vtb_r(contador_r) = Vtb;

%time_step = time_step + time_step;
contador_r = contador_r + 1;

t_0 = t1;
%gap_ini = gap;

end

% figure(1)
% plot(Salida.Vtb_r,-Salida.Itb_r)
% set(gca, 'YScale', 'log')
% title('Vtb vs Itb')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on
%
% figure(2)
% plot(tiempo,current_total)
% set(gca, 'YScale', 'log')
% title('Time vs Current')
% xlabel('Time (s)') % x-axis label
% ylabel('Current (A)') % y-axis label
% grid on
% 
% figure(3)
% plot(Salida.Vtb_r,Salida.Gap_r)
% set(gca, 'YScale', 'log')
% title('Voltage vs Gap')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on
% 
% figure(4)
% plot(Salida.Vtb_r,Salida.T_r)
% set(gca, 'YScale', 'log')
% title('Voltage vs T')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Gap (m)') % y-axis label
% grid on

%Se actualiza la barra de espera
try
    message = sprintf('Simulating. Please wait...');
    waitbar(100/100,wait_bar,message);
catch
    % si no se puede actualizar la barra de espera, es que
    % le hemos dado a cancelar, por lo que activamos la
    % variable measurement_stopped
    measurement_stopped = 1;
end

try
    % Se elimina la barra de espera
    delete(wait_bar);
end

%%%%%%%%%%%%%%%
%FIGURAS FINALES
%%%%%%%%%%%%%%%

Voltage = horzcat(Salida.Vtb,Salida.Vtb_r);
Current = horzcat(Salida.Itb,-Salida.Itb_r);

load Test 

axes(handles.axes1);
hold on
%plot(Salida.Vtb,Salida.Itb,'b',Salida.Vtb_r,-Salida.Itb_r,'b') 
plot (Voltage,Current,'b')
plot (VoltageSet,-CurrentSet,'--r',VoltageReset,CurrentReset,'--r')
hold off
set(gca, 'YScale', 'log')
title('Vtb vs Itb')
xlabel('Voltage (V)') % x-axis label
ylabel('Current (A)') % y-axis label
%legend(h(1:2),'Simulated Curve','Hspice Curve');
legend('Simulated Curve','Hspice Curve')
set(legend,'location','southwest')
grid on

axes(handles.axes2);
plot(Salida.Vtb,Salida.Gap*1e9,Salida.Vtb_r,Salida.Gap_r*1e9)
%set(gca, 'YScale', 'log')
title('Voltage vs Gap')
xlabel('Voltage (V)') % x-axis label
ylabel('Gap (nm)') % y-axis label
grid on

axes(handles.axes3);
plot(Salida.Vtb,Salida.T,Salida.Vtb_r,Salida.T_r)
%set(gca, 'YScale', 'log')
title('Voltage vs T')
xlabel('Voltage (V)') % x-axis label
ylabel('Temperature (K)') % y-axis label
grid on

% axes(handles.axes4);
% plot(Salida.Vtb,Salida.T,Salida.Vtb_r,Salida.T_r)
% %set(gca, 'YScale', 'log')
% title('Voltage vs T')
% xlabel('Voltage (V)') % x-axis label
% ylabel('Temperature (K)') % y-axis label
% grid on


% --- Executes on slider movement.
function slider_I0_Callback(hObject, eventdata, handles)
% hObject    handle to slider_I0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_I0 = get(hObject,'Value');
set(handles.edit_I0,'String',num2str(Slider_I0))

Parameters.I0 = Slider_I0;


% --- Executes during object creation, after setting all properties.
function slider_I0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_I0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end


% --- Executes on slider movement.
function slider_g0_Callback(hObject, eventdata, handles)
% hObject    handle to slider_g0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_g0 = get(hObject,'Value');
set(handles.edit_g0,'String',num2str(Slider_g0))

Parameters.g0 = Slider_g0;


% --- Executes during object creation, after setting all properties.
function slider_g0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_g0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end


% --- Executes on slider movement.
function slider_V0_Callback(hObject, eventdata, handles)
% hObject    handle to slider_V0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_V0 = get(hObject,'Value');
set(handles.edit_V0,'String',num2str(Slider_V0))

Parameters.V0 = Slider_V0;


% --- Executes during object creation, after setting all properties.
function slider_V0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_V0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end



function edit_g0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_g0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_g0 as text
%        str2double(get(hObject,'String')) returns contents of edit_g0 as a double

global Parameters

Slider_g0 = get(handles.edit_g0,'String'); % Obtaining manually introduced value
Slider_g0 = str2double(Slider_g0); % Transforming the value form string to double


Parameters.g0 = Slider_g0; % Saving the value

% --- Executes during object creation, after setting all properties.
function edit_g0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_g0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_V0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_V0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_V0 as text
%        str2double(get(hObject,'String')) returns contents of edit_V0 as a double

global Parameters

Slider_V0 = get(handles.edit_V0,'String'); % Obtaining manually introduced value
Slider_V0 = str2double(Slider_V0); % Transforming the value form string to double


Parameters.V0 = Slider_V0; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_V0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_V0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



function edit_I0_Callback(hObject, eventdata, handles)
% hObject    handle to edit_I0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_I0 as text
%        str2double(get(hObject,'String')) returns contents of edit_I0 as a double

global Parameters

Slider_I0 = get(handles.edit_I0,'String'); % Obtaining manually introduced value
Slider_I0 = str2double(Slider_I0); % Transforming the value form string to double


Parameters.I0 = Slider_I0; % Saving the value



% --- Executes during object creation, after setting all properties.
function edit_I0_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_I0 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end


% --- Executes on slider movement.
function slider_beta_Callback(hObject, eventdata, handles)
% hObject    handle to slider_beta (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_Beta = get(hObject,'Value');
set(handles.edit_beta,'String',num2str(Slider_Beta))

Parameters.Beta = Slider_Beta;


% --- Executes during object creation, after setting all properties.
function slider_beta_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_beta (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end



function edit_beta_Callback(hObject, eventdata, handles)
% hObject    handle to edit_beta (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_beta as text
%        str2double(get(hObject,'String')) returns contents of edit_beta as a double

global Parameters

Slider_Beta = get(handles.edit_beta,'String'); % Obtaining manually introduced value
Slider_Beta = str2double(Slider_Beta); % Transforming the value form string to double


%set(Slider_Gap_Min, 'Value', 0.25e-9); % Somewhere between max and min.


Parameters.Beta = Slider_Beta; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_beta_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_beta (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end



% --- Executes on slider movement.
function slider_gamma_Callback(hObject, eventdata, handles)
% hObject    handle to slider_gamma (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_Gamma = get(hObject,'Value');
set(handles.edit_gamma,'String',num2str(Slider_Gamma))

Parameters.Gamma = Slider_Gamma;

% --- Executes during object creation, after setting all properties.
function slider_gamma_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_gamma (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end



function edit_gamma_Callback(hObject, eventdata, handles)
% hObject    handle to edit_gamma (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_gamma as text
%        str2double(get(hObject,'String')) returns contents of edit_gamma as a double

global Parameters

Slider_Gamma = get(handles.edit_gamma,'String'); % Obtaining manually introduced value
Slider_Gamma = str2double(Slider_Gamma); % Transforming the value form string to double


Parameters.Gamma = Slider_Gamma; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_gamma_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_gamma (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end




% --- Executes on slider movement.
function slider_temp_limitation_min_Callback(hObject, eventdata, handles)
% hObject    handle to slider_temp_limitation_min (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_Temp_Min = get(hObject,'Value');
set(handles.edit_temp_limitation_min,'String',num2str(Slider_Temp_Min))

Parameters.Temp_Min = Slider_Temp_Min;


% --- Executes during object creation, after setting all properties.
function slider_temp_limitation_min_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_temp_limitation_min (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end



function edit_temp_limitation_min_Callback(hObject, eventdata, handles)
% hObject    handle to edit_temp_limitation_min (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_temp_limitation_min as text
%        str2double(get(hObject,'String')) returns contents of edit_temp_limitation_min as a double

global Parameters

Temp_Min = get(handles.edit_temp_limitation_min,'String'); % Obtaining manually introduced value
Temp_Min = str2double(Temp_Min); % Transforming the value form string to double


Parameters.Temp_Min = Temp_Min; % Saving the value


% --- Executes during object creation, after setting all properties.
function edit_temp_limitation_min_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_temp_limitation_min (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end


% --- Executes on slider movement.
function slider_temp_limitation_max_Callback(hObject, eventdata, handles)
% hObject    handle to slider_temp_limitation_max (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'Value') returns position of slider
%        get(hObject,'Min') and get(hObject,'Max') to determine range of slider

global Parameters

Slider_Temp_Max = get(hObject,'Value');
set(handles.edit_temp_limitation_max,'String',num2str(Slider_Temp_Max))

Parameters.Temp_Max = Slider_Temp_Max;



% --- Executes during object creation, after setting all properties.
function slider_temp_limitation_max_CreateFcn(hObject, eventdata, handles)
% hObject    handle to slider_temp_limitation_max (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: slider controls usually have a light gray background.
if isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor',[.9 .9 .9]);
end



function edit_temp_limitation_max_Callback(hObject, eventdata, handles)
% hObject    handle to edit_temp_limitation_max (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hints: get(hObject,'String') returns contents of edit_temp_limitation_max as text
%        str2double(get(hObject,'String')) returns contents of edit_temp_limitation_max as a double

global Parameters

Temp_Max = get(handles.edit_temp_limitation_max,'String'); % Obtaining manually introduced value
Temp_Max = str2double(Temp_Max); % Transforming the value form string to double


Parameters.Temp_Max = Temp_Max; % Saving the value



% --- Executes during object creation, after setting all properties.
function edit_temp_limitation_max_CreateFcn(hObject, eventdata, handles)
% hObject    handle to edit_temp_limitation_max (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    empty - handles not created until after all CreateFcns called

% Hint: edit controls usually have a white background on Windows.
%       See ISPC and COMPUTER.
if ispc && isequal(get(hObject,'BackgroundColor'), get(0,'defaultUicontrolBackgroundColor'))
    set(hObject,'BackgroundColor','white');
end


% --------------------------------------------------------------------
function Menu_Configuration_Callback(hObject, eventdata, handles)
% hObject    handle to Menu_Configuration (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)


% --------------------------------------------------------------------
function Menu_Configuration_Technological_Callback(hObject, eventdata, handles)
% hObject    handle to Menu_Configuration_Technological (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

Technological_Parameters


% --- Executes when user attempts to close figure1.
function figure1_CloseRequestFcn(hObject, eventdata, handles)
% hObject    handle to figure1 (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

% Hint: delete(hObject) closes the figure

global Parameters

save('Parameters.mat','Parameters');

delete(hObject);


% --------------------------------------------------------------------
function Menu_Configuration_Physical_Callback(hObject, eventdata, handles)
% hObject    handle to Menu_Configuration_Physical (see GCBO)
% eventdata  reserved - to be defined in a future version of MATLAB
% handles    structure with handles and user data (see GUIDATA)

Physical_Parameters
