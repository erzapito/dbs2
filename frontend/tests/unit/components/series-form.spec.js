import { shallowMount } from '@vue/test-utils';
import SeriesForm from '@/components/series-form.vue';
import moxios from 'moxios';

describe('series-form.vue', () => {

    beforeEach( () => {
        // import and pass your custom axios instance to this method
        moxios.install();
    });
    afterEach( () => {
        // import and pass your custom axios instance to this method
        moxios.uninstall();
    });

    it('renders', (done) => {
        const wrapper = shallowMount(SeriesForm,{
            propsData: {
                'item' : {
                    "id": 1,
                    "capitulos": "1-24",
                    "categoria": "Descargado",
                    "fansub": "HorribleSubs",
                    "idioma": "es",
                    "name": "test series 1"
                },
            }
        });
        expect(wrapper.find('edit-field-stub[label=name]').attributes('field')).toBe('test series 1');
        expect(wrapper.find('edit-field-stub[label=chapters]').attributes('field')).toBe('1-24');
        expect(wrapper.find('edit-field-stub[label=category]').attributes('field')).toBe('Descargado');
        expect(wrapper.find('edit-field-stub[label=fansub]').attributes('field')).toBe('HorribleSubs');
        expect(wrapper.find('edit-field-stub[label=idioma]').attributes('field')).toBe('es');
        done();
    });

    it('updates', async (done) => {
        const item = {
            "id": 1,
            "capitulos": "1-24",
            "categoria": "Descargado",
            "fansub": "HorribleSubs",
            "idioma": "es",
            "name": "test series 1"
        };
        const wrapper = shallowMount(SeriesForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.save');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/series/1');
            expect(request.config.method).toEqual('put');
            const requestBody = JSON.parse(request.config.data);
            expect(requestBody).toEqual(item);
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["series-saved"]).toBeTruthy();
                expect(wrapper.emitted()["series-saved"]).toEqual([[item]]);
                done();
            });
        });

    });

    it('creates', async (done) => {
        const item = {
            "capitulos": "1-24",
            "categoria": "Descargado",
            "fansub": "HorribleSubs",
            "idioma": "es",
            "name": "test series 1"
        };
        const wrapper = shallowMount(SeriesForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.save');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/series/');
            expect(request.config.method).toEqual('post');
            const requestBody = JSON.parse(request.config.data);
            expect(requestBody).toEqual(item);
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["series-saved"]).toBeTruthy();
                expect(wrapper.emitted()["series-saved"]).toEqual([[item]]);
                done();
            });
        });

    });

    it('deletes', async (done) => {
        const item = {
            "id" : "1",
            "capitulos": "1-24",
            "categoria": "Descargado",
            "fansub": "HorribleSubs",
            "idioma": "es",
            "name": "test series 1"
        };
        const wrapper = shallowMount(SeriesForm,{
            propsData: {
                'item' : item,
            }
        });
        const save_button = wrapper.find('.remove');
        save_button.trigger('click');

        await wrapper.vm.$nextTick()
        moxios.wait(() => {
            const request = moxios.requests.mostRecent();
            expect(request.config.url).toEqual('/api/series/1');
            expect(request.config.method).toEqual('delete');
            request.respondWith({
              status: 200,
            }).then(() => {
                expect(wrapper.emitted()["series-deleted"]).toBeTruthy();
                expect(wrapper.emitted()["series-deleted"]).toEqual([[item]]);
                done();
            });
        });

    });

});
